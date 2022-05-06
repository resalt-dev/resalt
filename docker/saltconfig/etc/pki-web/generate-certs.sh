#!/bin/bash

echo "Enter domain, e.g. localhost"
echo ""
echo -n "Domain: "

read DOMAIN
echo ""

echo "Generating Self-Signed certificate for domain: $DOMAIN"

export MSYS_NO_PATHCONV=1

if [ ! -f "rootca.crt" ]; then
	openssl genrsa -out rootca.key.pem 4096
	openssl req -x509 -new -nodes -key rootca.key.pem -sha256 -days 1024 -subj="/C=EU/ST=XX/O=Hibike/CN=HIBIKE-ROOT-CA" -out rootca.crt
fi

if [ ! -f "secret.key" ]; then
	openssl genrsa -out secret.key 2048
fi

# Create certificate request
openssl req -new -sha256 -key secret.key -subj "/C=EU/ST=XX/O=Hibike/CN=$DOMAIN" -out $DOMAIN.csr

# Sign it
openssl x509 -req -in $DOMAIN.csr -CA rootca.crt -CAkey rootca.key.pem -CAcreateserial -out $DOMAIN.crt -days 500 -sha256 -extfile v3.ext
rm $DOMAIN.csr
