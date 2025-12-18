# Test Targets

## Legal Testing Environments

**IMPORTANT:** Only test against these intentionally vulnerable applications or targets you have explicit permission to test.

### Recommended Practice Targets

1. **DVWA (Damn Vulnerable Web Application)**
   ```bash
   docker run -p 80:80 vulnerables/web-dvwa
   shadowprobe scan --url http://localhost
   ```

2. **bWAPP**
   ```bash
   docker run -p 8080:80 raesene/bwapp
   shadowprobe scan --url http://localhost:8080
   ```

3. **OWASP Juice Shop**
   ```bash
   docker run -p 3000:3000 bkimminich/juice-shop
   shadowprobe scan --url http://localhost:3000
   ```

4. **WebGoat**
   ```bash
   docker run -p 8080:8080 webgoat/webgoat-8.0
   shadowprobe scan --url http://localhost:8080/WebGoat
   ```

### Bug Bounty Platforms

- HackerOne (https://hackerone.com)
- Bugcrowd (https://bugcrowd.com)
- Intigriti (https://intigriti.com)
- YesWeHack (https://yeswehack.com)

### Public Bug Bounty Programs

Always read and follow the program rules!

## Example Scans

### Scan DVWA
```bash
# Start DVWA
docker run -d -p 80:80 --name dvwa vulnerables/web-dvwa

# Scan with ShadowProbe
docker compose run --rm shadowprobe scan \
  --url http://host.docker.internal \
  --aggressive \
  -o dvwa-results.json \
  --html dvwa-report.html
```

### Scan Juice Shop
```bash
# Start Juice Shop
docker run -d -p 3000:3000 --name juice-shop bkimminich/juice-shop

# Scan
docker compose run --rm shadowprobe scan \
  --url http://host.docker.internal:3000 \
  --depth 5 \
  -o juice-results.json
```

## Disclaimer

**USE THIS TOOL RESPONSIBLY AND LEGALLY**

- Only scan systems you own or have explicit permission to test
- Follow bug bounty program rules
- Do not perform DoS attacks
- Respect rate limits
- This tool is for educational and authorized security testing only
