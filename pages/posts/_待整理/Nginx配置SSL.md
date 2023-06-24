# Nginx配置SSL
## 使用Certbot
[certbot/certbot: Certbot is EFF's tool to obtain certs from Let's Encrypt and (optionally) auto-enable HTTPS on your server. It can also act as a client for any other CA that uses the ACME protocol. (github.com)](https://github.com/certbot/certbot)

[Certbot Instructions | Certbot (eff.org)](https://certbot.eff.org/instructions?ws=nginx&os=debianbuster)

```bash
aptitude install snapd
```

```bash
snap install core
snap refresh core
```

```bash
snap install --classic certbot
ln -s /snap/bin/certbot /usr/bin/certbot
```

```bash
certbot --nginx
```

```bash
nginx -s reload
```

测试自动续期证书
```bash
certbot renew --dry-run
```
