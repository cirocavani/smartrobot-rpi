# HTTPS, WSS, Custom CA configuration

Requirements:

- `mediaDevices` (Camera, Microphone) requires HTTPS
- `Websocket` uses WSS when using HTTPS

Implications:

- HTTP Server should use TLS configuration
- Signalling Server should use TLS configuration
- `webrtcsink` should connect to the Signalling Serverusing WSS

<https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mediaDevices>

> The mediaDevices read-only property of the Navigator interface returns a MediaDevices
> object, which provides access to connected media input devices like cameras and
> microphones, as well as screen sharing.
>
> Secure context: This feature is available only in secure contexts (HTTPS), in some or all
> supporting browsers.
>
> Firefox, Chrome, Safari, WevView requires HTTPS.

<https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API/Writing_WebSocket_client_applications#security_considerations>

> WebSockets should not be used in a mixed content environment; that is, you shouldn't
> open a non-secure WebSocket connection from a page loaded using HTTPS or vice versa. Most
> browsers now only allow secure WebSocket connections, and no longer support using them in
> insecure contexts.

Solution:

- Create a new CA (Certificate Authority) keys
- Create server keys signed by the new CA key
- Import CA Certificate on Linux (ca-certificates package)
- Import CA Certificate into Firefox

Files:

- `[machine_id]_ca.key`: custom CA private key
- `[machine_id]_ca.crt`: custom CA certificate (public key)
- `[machine_id]_server.p12`: Server Key Store (signed by the custom CA)


```sh
# PWD <- Project Home 'smartrobot-rpi/dev-projects/gst-webrtc/'

sudo apt install openssl

mkdir -p pki/
cd pki/


# Create a new CA keys


# machine_id = olivia-v1

openssl req \
-x509 \
-newkey rsa:4096 \
-sha256 \
-nodes \
-days 3650 \
-keyout olivia-v1_ca.key \
-out olivia-v1_ca.crt \
-subj '/CN=Olivia v1 CA'

openssl x509 -in olivia-v1_ca.crt -noout -text

# Certificate:
#     Data:
#         Version: 3 (0x2)
#         Serial Number:
#             47:97:a6:fc:ad:56:27:07:39:86:f5:85:73:75:06:6e:81:f8:bb:a2
#         Signature Algorithm: sha256WithRSAEncryption
#         Issuer: CN = Olivia v1 CA
#         Validity
#             Not Before: Mar 18 18:02:57 2025 GMT
#             Not After : Mar 16 18:02:57 2035 GMT
#         Subject: CN = Olivia v1 CA
#         Subject Public Key Info:
#             Public Key Algorithm: rsaEncryption
#                 Public-Key: (4096 bit)
#                 Modulus:
#                     00:9c:19:a4:f3:09:8c:1c:42:fd:a5:c0:16:b5:a5:
#                     28:2a:ca:69:cc:98:df:cc:1f:55:f6:48:b5:86:71:
#                     ff:a1:14:01:6a:f3:4f:72:43:2f:8c:cf:d2:1e:e5:
#                     aa:10:5a:b9:57:32:bf:2c:0e:58:5b:ec:8a:84:64:
#                     c5:45:4b:9f:5e:2c:b3:ab:7e:68:a6:be:0f:54:ce:
#                     1e:ad:55:50:33:c1:4b:01:a9:45:ae:fe:e1:cb:2f:
#                     71:db:80:9f:5a:99:ae:23:81:b7:0e:49:4c:91:07:
#                     da:c5:89:b0:1c:aa:47:26:8d:bf:d4:a1:41:14:3c:
#                     4c:06:4b:5b:27:46:37:a7:50:b8:9e:23:61:bf:3a:
#                     3b:e7:a6:83:46:b9:5d:00:e9:f5:f1:de:b1:b8:06:
#                     e1:b2:75:68:3f:4a:df:6a:9b:dc:fa:7c:d1:5b:6a:
#                     20:27:3f:66:f4:88:41:45:77:74:dd:b4:a7:e4:fc:
#                     fb:93:27:6e:5e:73:b6:47:7b:7d:f8:99:ba:2d:a1:
#                     34:0d:dc:e4:f8:a7:0b:bb:0f:f9:16:ef:f4:21:08:
#                     99:0b:ae:3b:9d:e5:9b:08:39:14:c7:d3:74:29:a3:
#                     e0:28:57:d4:b5:10:dc:b5:19:0d:23:7c:0f:e7:a3:
#                     f6:67:40:fc:d4:b8:b9:34:2a:b2:52:9f:83:38:fa:
#                     f0:00:a6:ed:40:6c:c9:eb:a1:d6:a0:31:13:d2:f9:
#                     fa:af:5a:dd:ed:19:69:06:94:4d:9e:e9:38:57:0c:
#                     e3:4c:2b:5d:e2:3a:f7:6c:b6:60:7d:f2:3f:5b:f5:
#                     a8:52:aa:54:6c:bd:da:8b:c6:d7:d9:7f:f9:40:83:
#                     70:04:68:95:49:73:96:ca:ab:5a:29:48:af:b9:66:
#                     df:c5:ad:79:ee:c6:24:6a:3d:6a:02:52:b9:42:b3:
#                     5a:fd:c2:0e:44:27:13:61:50:c5:95:d1:b2:0b:14:
#                     97:1c:ad:cd:e1:fa:c2:c1:e0:a9:8b:c6:60:1c:1e:
#                     1b:e8:da:5f:0c:2f:68:55:53:1f:68:b4:71:7f:31:
#                     ba:4d:59:f2:f2:20:a2:34:93:31:be:bd:07:bb:1b:
#                     de:4a:2f:54:c2:58:5a:e0:74:70:57:23:6c:71:ea:
#                     b7:e5:59:50:00:2b:e3:9e:d0:0e:ca:98:6b:64:d8:
#                     6b:e9:38:96:b4:66:17:3e:8a:03:c6:7d:35:a8:80:
#                     d6:6e:0d:5a:ea:7c:56:56:40:89:f1:8c:16:c0:7a:
#                     21:54:20:7d:e0:60:d5:fe:75:dc:11:37:bf:48:83:
#                     fd:86:7b:e3:4b:7a:cb:f9:5a:34:52:38:e7:74:69:
#                     15:04:01:49:f4:80:ce:77:9d:f0:05:4a:bb:3d:b2:
#                     7d:97:91
#                 Exponent: 65537 (0x10001)
#         X509v3 extensions:
#             X509v3 Subject Key Identifier:
#                 72:EF:56:AB:4B:FC:C0:F6:84:D1:DF:19:C4:00:A6:37:CB:00:8A:9B
#             X509v3 Authority Key Identifier:
#                 72:EF:56:AB:4B:FC:C0:F6:84:D1:DF:19:C4:00:A6:37:CB:00:8A:9B
#             X509v3 Basic Constraints: critical
#                 CA:TRUE
#     Signature Algorithm: sha256WithRSAEncryption
#     Signature Value:
#         34:b9:9d:61:07:53:4a:d5:87:05:cb:93:11:72:ad:da:35:21:
#         46:e0:3a:16:aa:4f:c9:d7:74:81:bb:56:01:6c:5e:6d:10:c2:
#         f6:a2:57:12:56:7d:04:80:5c:40:25:ee:6f:85:88:36:31:a1:
#         c8:7d:90:e1:d1:7b:00:d7:00:cf:87:7b:36:f0:bd:76:ce:cf:
#         43:62:6e:30:b9:e1:d2:67:e0:06:df:bc:8c:0b:0a:7d:fd:07:
#         5f:67:b8:4c:39:73:d5:d4:38:f0:92:bb:0e:c3:1e:b3:06:9e:
#         f5:90:ca:c6:8a:4e:2b:30:59:39:4b:00:94:7f:e9:a2:72:80:
#         ea:db:44:69:74:b3:31:f6:f0:59:d7:ba:e5:e0:68:8b:75:f7:
#         e6:e3:d5:56:37:e9:fb:eb:5e:77:16:25:10:15:1f:de:9e:b6:
#         b8:66:6e:4a:1c:8a:97:8c:f0:3c:7c:81:f7:80:c1:d1:03:cd:
#         9f:a2:d5:5a:61:bf:7d:f4:da:a7:a9:89:80:4d:e5:84:ea:a4:
#         ed:b2:90:a4:9f:30:23:ab:25:6b:2a:1a:46:ae:53:11:84:73:
#         24:75:c7:f6:72:54:79:e6:6d:83:84:c5:aa:cb:6f:69:fa:fb:
#         01:be:ff:dd:20:b2:fc:48:f9:e7:4f:1a:ec:be:81:4b:6b:d3:
#         d0:f9:96:53:26:80:80:e8:08:92:66:d4:31:d0:49:1d:6f:ee:
#         7d:5a:50:f9:7b:81:55:7d:bd:90:72:b9:d4:da:2f:58:56:6d:
#         25:0a:1e:40:83:2e:90:85:97:3a:ec:48:6f:6b:12:be:13:6a:
#         78:f4:85:29:46:c8:1d:4b:16:b2:c1:07:ea:37:4c:12:f6:a8:
#         f2:2a:af:f1:a9:a9:43:5c:ea:26:e3:1f:36:3f:80:01:4d:bc:
#         36:93:a7:6d:53:11:af:ab:a9:9c:ce:c0:f8:5f:f7:16:86:74:
#         f2:2a:a6:66:96:ab:11:e9:e2:be:2c:67:64:f8:1a:96:6e:7e:
#         cb:f6:0a:68:4f:53:f3:af:05:33:30:19:e5:8e:b7:c3:6b:24:
#         d6:70:f0:f1:75:1c:85:e5:0c:ce:33:b1:f9:90:82:9a:8e:ab:
#         ac:ca:16:ca:49:0a:fc:34:fa:40:2b:54:e5:5c:5d:e6:95:18:
#         8a:d3:51:80:00:cf:b2:3e:d3:21:de:bd:b0:7b:a7:c5:03:31:
#         32:2d:08:95:5d:50:9a:9c:35:5c:a7:31:59:cd:0c:80:be:1e:
#         5a:36:95:4e:7f:3d:85:32:64:f3:f4:25:73:67:11:98:80:83:
#         a7:e3:40:b9:f7:7a:b3:b7:9e:95:15:4a:11:54:8e:6e:69:53:
#         f3:29:d8:e5:71:2d:fb:e


# Create the server keys (signed by the custom CA)


# CSR (Certificate Signing Request)

openssl req \
-newkey rsa:4096 \
-sha256 \
-nodes \
-keyout olivia-v1_server.key \
-out olivia-v1_server.csr \
-subj '/CN=*.olivia-v1.machine-domain'


# Subject Alternative Names
# https://man.openbsd.org/x509v3.cnf.5#Subject_alternative_name

# Firefox
# Error code: SSL_ERROR_BAD_CERT_DOMAIN
# Firefox does not trust this site because it uses a certificate that is not valid for
# webrtc.olivia-v1.machine-domain:8000.

echo 'subjectAltName=DNS:webrtc.olivia-v1.machine-domain,IP:192.168.72.123' \
> olivia-v1_server.cnf


# CA Signing

openssl x509 \
-req \
-days 3650 \
-in olivia-v1_server.csr \
-CA olivia-v1_ca.crt \
-CAkey olivia-v1_ca.key \
-CAcreateserial \
-extfile olivia-v1_server.cnf \
-out olivia-v1_server.crt

openssl x509 -in olivia-v1_server.crt -noout -text

# Certificate:
#     Data:
#         Version: 3 (0x2)
#         Serial Number:
#             49:c1:ff:8b:80:cb:89:17:56:8d:d7:81:2c:cd:2e:91:d8:af:7e:3c
#         Signature Algorithm: sha256WithRSAEncryption
#         Issuer: CN = Olivia v1 CA
#         Validity
#             Not Before: Mar 20 11:37:58 2025 GMT
#             Not After : Mar 18 11:37:58 2035 GMT
#         Subject: CN = *.olivia-v1.machine-domain
#         Subject Public Key Info:
#             Public Key Algorithm: rsaEncryption
#                 Public-Key: (4096 bit)
#                 Modulus:
#                     00:ec:08:c4:75:3d:f7:63:e0:23:e2:3d:5a:c9:00:
#                     bd:6e:00:d2:4b:b6:ac:49:7f:fb:20:7f:2a:af:14:
#                     3f:e2:97:e3:8c:a4:ad:c3:cb:31:54:58:e5:f1:ee:
#                     db:90:9d:32:96:c8:09:29:be:48:98:e0:56:26:f2:
#                     53:b2:4b:cc:ab:c4:39:83:ba:dc:21:46:80:fa:51:
#                     5f:cd:65:2a:22:c2:e0:c7:4d:3d:66:bc:5b:70:25:
#                     0b:6c:23:88:73:ff:2d:f4:bb:30:08:05:0b:9f:65:
#                     58:57:de:93:ee:73:dd:4d:be:b7:9d:0a:74:12:c2:
#                     81:9a:a7:a0:5e:ff:51:5b:ee:81:e7:3f:52:be:10:
#                     70:1b:56:d8:b0:de:39:76:f6:ac:d8:d9:bb:f4:f0:
#                     62:1a:b6:e2:47:2a:f9:1a:42:6b:aa:82:fa:28:78:
#                     d5:f0:da:72:c4:f6:97:3d:92:18:5e:1a:ca:b8:91:
#                     22:6f:5c:d7:1d:f9:65:e2:ab:78:9c:70:95:77:5e:
#                     d8:ba:7b:0a:4e:c7:3e:78:d2:00:0d:fb:c3:ba:cf:
#                     95:24:c5:a0:77:e1:f0:1b:92:16:d8:b5:ab:50:2c:
#                     26:a8:fe:d7:b2:c7:c6:23:de:d3:bf:53:3d:13:97:
#                     3c:8c:5f:f3:53:86:e3:e0:9f:45:f0:c8:02:47:e3:
#                     9b:96:cd:ae:c7:85:b2:1e:c2:a3:a3:0e:2d:0a:1e:
#                     6d:4f:34:6a:ac:89:75:b7:05:25:a3:97:72:bf:06:
#                     30:ca:b2:e5:fd:1d:53:73:bb:a4:4d:07:10:e0:28:
#                     df:12:b9:9e:2d:88:fd:33:2d:a6:ea:39:5b:b8:e0:
#                     44:4d:e2:e8:c5:cc:5f:bc:d2:4d:f6:63:bc:79:42:
#                     10:ef:2b:a0:3a:66:62:9f:5a:08:a8:0f:93:df:ba:
#                     ec:90:e1:95:d2:5a:c5:f2:fa:39:3b:8e:28:10:5b:
#                     f8:70:39:45:11:53:96:34:17:3a:b6:71:e8:9d:2e:
#                     7d:e2:e6:5b:5c:0e:a7:51:f4:70:7a:45:69:7b:44:
#                     7e:81:7d:b1:d0:1a:68:42:b6:56:fb:d4:98:a5:2b:
#                     28:b9:13:ab:dc:14:c1:08:3b:62:77:0d:e5:09:13:
#                     e4:61:8b:95:c2:7c:e6:4a:1d:f7:6b:b3:05:3c:d6:
#                     a6:23:96:4f:f0:39:ad:c1:b6:5e:21:39:4b:69:45:
#                     79:4f:2d:cd:bb:85:79:63:c1:79:01:52:4d:65:31:
#                     53:f0:49:de:4e:49:51:72:d9:78:ac:a0:3a:48:ff:
#                     51:45:ab:06:8e:9e:4e:63:0d:d2:e1:c8:ff:09:76:
#                     96:04:49:39:62:ee:a7:51:32:a3:a2:e5:33:b3:0a:
#                     07:5f:73
#                 Exponent: 65537 (0x10001)
#         X509v3 extensions:
#             X509v3 Subject Alternative Name:
#                 DNS:webrtc.olivia-v1.machine-domain, IP Address:192.168.72.123
#             X509v3 Subject Key Identifier:
#                 36:CA:B3:27:C5:60:5D:A3:7F:EA:D6:61:DB:1E:A7:26:F7:D6:8F:52
#             X509v3 Authority Key Identifier:
#                 7A:82:31:01:C6:A3:B2:C9:C7:4E:08:F8:D2:7B:34:85:BF:A5:F3:74
#     Signature Algorithm: sha256WithRSAEncryption
#     Signature Value:
#         8c:9e:a1:40:3e:ee:c5:53:cd:c2:13:bc:42:3a:bf:36:74:29:
#         7c:50:87:86:9f:84:ed:99:e8:b4:07:98:00:9c:3e:b6:ab:1b:
#         28:8c:9a:2a:cc:cc:9b:f0:5e:9b:7f:b8:19:5d:45:d4:91:3e:
#         78:dc:ac:3c:62:ca:69:dd:cb:aa:54:e2:13:6f:fd:74:44:19:
#         1f:52:c1:85:1d:4c:ba:a4:c3:04:d1:b6:f8:43:c7:a3:7e:05:
#         d4:94:ec:92:ec:83:78:21:e6:ab:26:b3:f9:d5:7e:76:d9:e3:
#         99:67:62:7a:71:7f:76:ca:4d:f1:b9:23:a0:fb:47:00:15:b0:
#         21:8a:cb:7c:fa:e7:63:de:95:f7:02:f2:d1:4e:3c:40:4f:48:
#         b7:87:0a:67:c6:aa:a1:98:9d:fa:cb:40:3b:0e:47:5d:75:94:
#         95:42:3e:14:0c:4e:a0:cf:ba:00:27:2b:a0:23:60:75:5d:2d:
#         3b:89:d2:ae:cf:e7:9b:1c:c0:bb:51:9a:8c:99:03:20:a8:32:
#         0f:d8:14:ee:29:f3:53:1f:95:a8:bd:bd:e5:90:58:de:22:b1:
#         c1:82:5f:06:86:f1:c9:1f:87:4c:86:f9:13:30:de:70:5d:3a:
#         f3:31:8f:c7:70:a8:f1:6b:c7:3b:b0:12:c1:b0:01:e2:ed:73:
#         7c:51:82:e3:31:f6:9b:e9:75:d4:a7:71:7e:75:15:09:09:49:
#         13:55:a6:43:eb:98:df:fd:c8:19:c1:56:fc:0e:c6:73:c7:6a:
#         bb:6b:ae:2f:e3:89:8a:b3:f0:48:7e:4f:f5:43:01:a8:c6:26:
#         e4:79:09:28:59:53:17:10:ba:51:dd:67:ac:ef:60:5e:32:af:
#         a7:d1:c7:0c:7e:48:6b:64:14:a7:da:48:f4:87:14:ee:9b:83:
#         7f:bf:6d:7d:7a:1d:16:4e:15:a9:5c:04:01:59:c1:43:82:0c:
#         f7:5e:eb:d2:fb:2e:75:d9:e1:d2:95:de:b8:d1:db:08:3a:00:
#         90:49:0d:a2:c9:1e:8f:5d:31:e3:96:2a:87:ea:21:ca:1b:4c:
#         3c:14:be:90:b6:63:96:cc:dc:90:e8:dd:6e:8e:27:9e:df:17:
#         33:1a:1f:f6:45:b3:46:52:5d:8d:05:dd:ea:a4:5d:66:d5:47:
#         3a:b4:d9:40:0a:2d:7d:5a:33:f8:a8:81:31:ff:bf:3d:39:8b:
#         07:1d:a1:48:a6:3b:6d:c7:b2:b7:f6:38:70:5e:84:10:40:9d:
#         90:97:d2:3d:36:28:c8:b8:b1:ff:8f:a0:d3:df:9e:e9:cb:f7:
#         b5:4b:e6:32:fd:42:89:fb:93:ed:61:02:7b:10:cf:b4:d5:53:
#         92:18:e8:3a:da:96:b2:57


# Server key store file (PKCS#12)

openssl pkcs12 \
-export \
-out olivia-v1_server.p12 \
-passout 'pass:' \
-inkey olivia-v1_server.key \
-in olivia-v1_server.crt


# Files

ls -alh olivia-v1_*

# -rw-r--r-- 1 root root 1.8K Mar 18 18:02 olivia-v1_ca.crt
# -rw------- 1 root root 3.2K Mar 18 18:02 olivia-v1_ca.key
# -rw-r--r-- 1 root root   41 Mar 18 18:06 olivia-v1_ca.srl
# -rw-r--r-- 1 root root   51 Mar 18 18:05 olivia-v1_server.cnf
# -rw-r--r-- 1 root root 1.9K Mar 18 18:06 olivia-v1_server.crt
# -rw-r--r-- 1 root root 1.6K Mar 18 18:05 olivia-v1_server.csr
# -rw------- 1 root root 3.2K Mar 18 18:05 olivia-v1_server.key
# -rw------- 1 root root 4.2K Mar 18 18:09 olivia-v1_server.p12


# Debian / Ubuntu - Install Root CA Certificate


sudo mkdir -p /usr/local/share/ca-certificates/olivia-v1

sudo cp olivia-v1_ca.crt /usr/local/share/ca-certificates/olivia-v1/

sudo update-ca-certificates

# Updating certificates in /etc/ssl/certs...
# 1 added, 0 removed; done.
# Running hooks in /etc/ca-certificates/update.d...
# done


# Firefox - Import Root CA Certificate

# 1. Open Settings
# 2. Open Privacy & Security
# 3. Go to Certificates and open View Certificates...
# 4. Go to Authorities and open Import...
# 5. Select olivia-v1_ca.crt
# 6. Check Trust this CA to identify websites.
# 7. Ok


# Server hostname resolution
# (required to validate server certificate)
# Server Certificate: webrtc.olivia-v1.machine-domain

# On Server

sudo sed -i '0,/localhost/s//localhost webrtc.olivia-v1.machine-domain/' /etc/hosts

# On Client
# 192.168.72.123 <- Server IP Address

echo '

# Olivia v1 WebRTC Server (TLS hostname)
192.168.72.123 webrtc.olivia-v1.machine-domain' | \
sudo tee -a /etc/hosts


# Test

# Start HTTP Server with TLS

# cargo binstall simple-http-server

simple-http-server --cert olivia-v1_server.p12

#      Index: disabled, Cache: enabled, Cors: disabled, Coop: disabled, Coep: disabled, Range:
#           Upload: disabled, CSRF Token:
#           Auth: disabled, Compression: disabled
#          https: enabled, Cert: olivia-v1_server.p12, Cert-Password:
#           Root: /home/cavani/Workspace/smartrobot-rpi/dev-projects/gst-webrtc/pki,
#     TryFile404:
#        Address: https://0.0.0.0:8000
#     ======== [2025-03-18 13:50:40] ========
# [2025-03-18 13:50:50] - 192.168.72.152 - 200 - HEAD /


# Run HTTP Client

curl -vI https://webrtc.olivia-v1.machine-domain:8000/

# * Host webrtc.olivia-v1.machine-domain:8000 was resolved.
# * IPv6: (none)
# * IPv4: 192.168.72.123
# *   Trying 192.168.72.123:8000...
# * Connected to webrtc.olivia-v1.machine-domain (192.168.72.123) port 8000
# * ALPN: curl offers h2,http/1.1
# * TLSv1.3 (OUT), TLS handshake, Client hello (1):
# *  CAfile: /etc/ssl/certs/ca-certificates.crt
# *  CApath: /etc/ssl/certs
# * TLSv1.3 (IN), TLS handshake, Server hello (2):
# * TLSv1.2 (IN), TLS handshake, Certificate (11):
# * TLSv1.2 (IN), TLS handshake, Server key exchange (12):
# * TLSv1.2 (IN), TLS handshake, Server finished (14):
# * TLSv1.2 (OUT), TLS handshake, Client key exchange (16):
# * TLSv1.2 (OUT), TLS change cipher, Change cipher spec (1):
# * TLSv1.2 (OUT), TLS handshake, Finished (20):
# * TLSv1.2 (IN), TLS handshake, Finished (20):
# * SSL connection using TLSv1.2 / ECDHE-RSA-CHACHA20-POLY1305 / x25519 / RSASSA-PSS
# * ALPN: server did not agree on a protocol. Uses default.
# * Server certificate:
# *  subject: CN=*.olivia-v1.machine-domain
# *  start date: Mar 18 16:47:47 2025 GMT
# *  expire date: Mar 16 16:47:47 2035 GMT
# *  common name: *.olivia-v1.machine-domain (matched)
# *  issuer: CN=Olivia v1 CA
# *  SSL certificate verify ok.
# *   Certificate level 0: Public key type RSA (4096/152 Bits/secBits), signed using sha256WithRSAEncryption
# *   Certificate level 1: Public key type RSA (4096/152 Bits/secBits), signed using sha256WithRSAEncryption
# * using HTTP/1.x
# > HEAD / HTTP/1.1
# > Host: webrtc.olivia-v1.machine-domain:8000
# > User-Agent: curl/8.9.1
# > Accept: */*
# >
# * Request completely sent off
# < HTTP/1.1 200 OK
# HTTP/1.1 200 OK
# < Content-Length: 1987
# Content-Length: 1987
# < Content-Type: text/html; charset=utf-8
# Content-Type: text/html; charset=utf-8
# < Date: Tue, 18 Mar 2025 16:50:50 GMT
# Date: Tue, 18 Mar 2025 16:50:50 GMT
# <
# 
# * shutting down connection #0
# * TLSv1.2 (OUT), TLS alert, close notify (256):


# Firefox

firefox --private-window https://webrtc.olivia-v1.machine-domain:8000/

# Should open with no security warnings
```

