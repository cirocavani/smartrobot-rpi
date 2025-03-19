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
# PWD <- Project Home 'programmable-matter-rpi/dev-projects/gst-webrtc/'

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

echo 'subjectAltName=DNS:webrtc.olivia-v1.machine-domain' > olivia-v1_server.cnf


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
#             06:7a:6b:91:85:06:82:50:f6:4b:7b:aa:71:98:d1:da:49:6d:a4:11
#         Signature Algorithm: sha256WithRSAEncryption
#         Issuer: CN = Olivia v1 CA
#         Validity
#             Not Before: Mar 18 18:06:15 2025 GMT
#             Not After : Mar 16 18:06:15 2035 GMT
#         Subject: CN = *.olivia-v1.machine-domain
#         Subject Public Key Info:
#             Public Key Algorithm: rsaEncryption
#                 Public-Key: (4096 bit)
#                 Modulus:
#                     00:a9:9b:58:06:3a:16:a8:d8:81:0e:c3:0b:a1:2d:
#                     0c:3f:2f:56:47:bb:85:c3:46:bb:91:a9:a4:e4:ae:
#                     ab:1b:df:e5:6f:d7:41:6c:61:78:3f:b4:a2:82:81:
#                     fa:f5:81:bc:9d:cd:ba:8e:c4:da:95:56:5b:56:4c:
#                     54:a3:f4:57:a2:49:46:3d:68:d4:1d:b5:a9:35:7b:
#                     c4:35:21:90:a8:f2:c0:4a:e0:9e:f5:ce:1d:97:88:
#                     16:4e:fe:c6:42:82:af:de:84:1f:1c:53:25:0c:2a:
#                     98:a3:df:1c:8c:32:f3:3a:5a:2d:6c:74:c4:ea:69:
#                     8e:61:56:31:c1:8b:1d:8b:0d:2d:dd:bd:23:02:c5:
#                     8d:ba:c1:98:52:d4:5b:57:55:98:2f:b7:08:e7:f7:
#                     6a:4f:d8:b1:2e:e0:f0:78:d7:81:e2:c0:bf:26:52:
#                     0c:f4:09:a5:09:35:ad:40:23:c8:2f:0a:0f:b8:60:
#                     db:ba:4b:f0:ea:f9:ef:6b:77:b7:ef:61:90:ad:ca:
#                     d4:1d:95:62:4b:31:f3:b3:64:ba:31:07:3b:00:ec:
#                     9a:d2:7e:32:96:37:62:77:35:e4:2f:9f:d8:d7:f0:
#                     f3:ad:f8:e6:49:3f:ed:5a:29:04:8a:42:fc:4b:fc:
#                     45:b1:3e:1a:a6:e3:a4:46:37:6c:ec:1f:00:7b:21:
#                     5e:70:46:28:14:a0:7d:a5:14:53:4b:0f:c2:f1:1b:
#                     db:30:db:4e:87:a9:84:ef:72:ec:8c:2e:52:5f:44:
#                     e9:7b:7f:b0:26:f7:2d:65:45:cc:e8:ef:78:58:4e:
#                     c9:1d:19:6b:e4:6e:1e:69:fe:e7:91:f1:5f:8d:c1:
#                     d9:c0:e1:04:b0:62:76:45:66:e2:b6:6b:ca:fd:77:
#                     73:c2:38:66:d4:95:df:fa:ac:cb:8f:11:ee:85:74:
#                     17:f6:33:00:53:84:a1:04:64:36:f3:8e:a8:27:6b:
#                     aa:8c:88:91:c1:17:04:55:07:ac:52:96:a1:54:dd:
#                     31:27:c5:0d:a0:3d:b0:5f:e0:62:01:b2:95:4c:24:
#                     e4:98:5c:47:c2:e5:60:29:73:38:9d:c7:ff:41:2a:
#                     f5:71:8c:c3:27:9e:dd:04:2b:2e:50:89:2f:2a:47:
#                     af:6f:3a:1d:2d:38:b3:90:e2:cb:3a:fa:d2:a6:18:
#                     5c:1e:f7:35:b0:16:18:ec:ce:e6:b1:46:57:6a:9b:
#                     2b:45:a3:b6:7a:bf:27:93:86:41:2c:4e:00:eb:d3:
#                     24:80:09:bb:65:e1:e4:f8:94:16:3f:55:b5:bd:15:
#                     bb:e0:67:f5:a2:ea:01:e7:86:cc:69:17:c6:12:00:
#                     bd:ae:0f:11:cd:a6:5d:a1:74:2d:ab:53:ec:81:71:
#                     0d:d3:8f
#                 Exponent: 65537 (0x10001)
#         X509v3 extensions:
#             X509v3 Subject Alternative Name:
#                 DNS:webrtc.olivia-v1.machine-domain
#             X509v3 Subject Key Identifier:
#                 07:14:B3:C9:33:1B:98:41:46:3E:8E:DB:68:6D:60:18:E8:03:41:AA
#             X509v3 Authority Key Identifier:
#                 72:EF:56:AB:4B:FC:C0:F6:84:D1:DF:19:C4:00:A6:37:CB:00:8A:9B
#     Signature Algorithm: sha256WithRSAEncryption
#     Signature Value:
#         42:bc:5b:55:2f:2a:0b:7e:7c:66:7e:28:78:4c:43:5d:de:0c:
#         b7:1a:94:b0:2d:38:71:e6:10:f9:c8:43:2b:83:d5:0e:dd:8f:
#         8d:35:b2:c7:b7:5b:9c:f1:e6:f8:c8:3e:2d:a9:52:d4:28:8d:
#         97:42:8f:f2:f1:f2:5c:71:b1:df:ef:2e:9a:71:f2:06:74:10:
#         10:25:31:9d:af:b6:83:6c:6f:c4:fb:e4:ce:35:f2:cc:ca:54:
#         26:18:f4:0e:df:94:b1:02:ea:ad:8a:50:9e:54:91:4a:c3:d8:
#         9a:62:ff:eb:ee:18:c5:0f:79:47:ff:bd:a1:e9:87:b8:ca:4c:
#         2b:41:c5:9b:74:12:b8:fb:96:4f:d4:96:09:f0:77:e3:eb:e9:
#         e5:8e:a6:f4:b2:bb:90:9c:d9:7b:80:fa:bc:9f:6e:f1:c3:ff:
#         d7:36:13:b8:ab:07:5b:42:19:62:00:14:73:62:10:26:e5:96:
#         f1:11:ed:a4:1d:bb:d0:53:bc:43:52:b8:e1:ec:38:6e:c9:21:
#         66:14:8d:f3:bb:f3:26:9a:d4:46:87:cd:79:ad:fb:b1:3b:95:
#         02:7a:57:bb:1f:fb:9b:7c:32:78:d1:d4:f7:f7:43:de:d1:42:
#         51:fe:e6:fc:36:78:7b:36:00:ad:c8:5d:f4:78:45:1b:ff:37:
#         2c:47:8b:4a:ef:cc:5c:72:51:1c:4f:de:d8:37:3c:af:6f:10:
#         24:83:64:d7:ce:c4:15:3e:4f:93:93:f8:f1:34:30:ee:a5:24:
#         be:a2:e0:89:6c:6f:0a:30:f9:be:4a:c9:7f:c1:49:3f:7c:5e:
#         ec:f4:51:20:2d:c9:f7:b0:15:93:00:02:24:7c:dc:7a:e9:f4:
#         01:9b:10:67:e4:ad:c5:80:41:0e:4d:da:3d:47:70:4a:df:cf:
#         46:ba:ac:8f:42:36:77:f0:63:af:0f:50:43:24:dd:36:96:f4:
#         8c:bc:b0:67:5b:32:b1:aa:f8:6e:30:3f:38:a2:90:56:63:4b:
#         a1:11:0b:e4:c6:b8:53:b1:72:55:84:43:8d:69:f3:eb:26:4e:
#         03:63:9b:8c:48:e6:c9:39:2d:fa:5b:00:9b:3f:02:54:18:8b:
#         72:2e:65:94:ef:9b:e6:b1:54:06:0f:f1:5b:f4:d1:39:b5:86:
#         d6:e4:29:bc:00:bd:0e:c1:42:ae:7d:64:95:24:24:ca:de:90:
#         fd:8b:1c:d3:f4:12:2f:ec:45:1d:b2:87:59:f6:59:da:8a:89:
#         47:9c:35:de:b9:ae:73:99:73:97:66:dc:85:66:62:7e:83:9f:
#         0e:b9:1f:94:0e:1d:04:d6:45:ce:bf:ff:27:fd:b3:a2:08:71:
#         ea:9f:57:78:51:6f:27:37


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
#           Root: /home/cavani/Workspace/programmable-matter-rpi/dev-projects/gst-webrtc/pki,
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

