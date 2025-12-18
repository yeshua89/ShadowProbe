use crate::client::{HttpClient, HttpResponse};
use shadowprobe_core::{Result, ShadowProbeError};
use scraper::{Html, Selector};
use url::Url;
use dashmap::DashSet;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tracing::{debug, info, warn};

pub struct Crawler {
    client: HttpClient,
    base_url: Url,
    visited: Arc<DashSet<String>>,
    discovered: Arc<DashSet<String>>,
    max_depth: usize,
    max_concurrent: usize,
}

impl Crawler {
    pub fn new(
        client: HttpClient,
        base_url: &str,
        max_depth: usize,
        max_concurrent: usize,
    ) -> Result<Self> {
        let base_url = Url::parse(base_url)
            .map_err(|e| ShadowProbeError::InvalidUrl(e.to_string()))?;

        Ok(Self {
            client,
            base_url,
            visited: Arc::new(DashSet::new()),
            discovered: Arc::new(DashSet::new()),
            max_depth,
            max_concurrent,
        })
    }

    pub async fn crawl(&self) -> Result<Vec<String>> {
        info!("Starting crawl of {}", self.base_url);

        let semaphore = Arc::new(Semaphore::new(self.max_concurrent));
        self.crawl_recursive(self.base_url.as_str(), 0, semaphore).await?;

        let urls: Vec<String> = self.discovered.iter()
            .map(|r| r.clone())
            .collect();

        info!("Crawling complete. Discovered {} unique URLs", urls.len());
        Ok(urls)
    }

    fn crawl_recursive<'a>(
        &'a self,
        url: &'a str,
        depth: usize,
        semaphore: Arc<Semaphore>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move {
            if depth > self.max_depth {
                return Ok(());
            }

            if self.visited.contains(url) {
                return Ok(());
            }

            self.visited.insert(url.to_string());
            self.discovered.insert(url.to_string());

            debug!("Crawling {} (depth: {})", url, depth);

            let _permit = semaphore.acquire().await
                .map_err(|e| ShadowProbeError::HttpError(e.to_string()))?;

            let response = match self.client.get(url).await {
                Ok(resp) => resp,
                Err(e) => {
                    warn!("Failed to fetch {}: {}", url, e);
                    return Ok(());
                }
            };

            if !response.is_html() {
                return Ok(());
            }

            let links = self.extract_links(&response.body, url)?;

            drop(_permit);

            let mut tasks = vec![];
            for link in links {
                if !self.visited.contains(&link) && self.is_same_domain(&link) {
                    let sem = semaphore.clone();
                    tasks.push(self.crawl_recursive(&link, depth + 1, sem));
                }
            }

            for task in tasks {
                let _ = task.await;
            }

            Ok(())
        })
    }

    fn extract_links(&self, html: &str, base_url: &str) -> Result<Vec<String>> {
        let document = Html::parse_document(html);
        let selectors = vec![
            Selector::parse("a[href]").unwrap(),
            Selector::parse("form[action]").unwrap(),
            Selector::parse("script[src]").unwrap(),
            Selector::parse("link[href]").unwrap(),
        ];

        let mut links = Vec::new();
        let base = Url::parse(base_url)
            .map_err(|e| ShadowProbeError::InvalidUrl(e.to_string()))?;

        for selector in selectors {
            for element in document.select(&selector) {
                let attr = if element.value().name() == "a" || element.value().name() == "link" {
                    "href"
                } else if element.value().name() == "form" {
                    "action"
                } else {
                    "src"
                };

                if let Some(href) = element.value().attr(attr) {
                    if let Ok(absolute_url) = base.join(href) {
                        let url_str = absolute_url.to_string();
                        // Remove fragments
                        let clean_url = url_str.split('#').next().unwrap_or(&url_str);
                        if !clean_url.is_empty() {
                            links.push(clean_url.to_string());
                        }
                    }
                }
            }
        }

        Ok(links)
    }

    fn is_same_domain(&self, url: &str) -> bool {
        if let Ok(parsed) = Url::parse(url) {
            parsed.domain() == self.base_url.domain()
        } else {
            false
        }
    }

    pub fn get_discovered_urls(&self) -> Vec<String> {
        self.discovered.iter().map(|r| r.clone()).collect()
    }
}
