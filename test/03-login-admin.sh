#!/bin/bash
RESPONSE=$(curl -s -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin123"}')

# Simpan token ke file sementara
TOKEN=$(echo "$RESPONSE" | grep -o '"token":"[^"]*"' | cut -d'"' -f4)
if [ -z "$TOKEN" ] || [ "$TOKEN" = "null" ]; then
  echo "❌ Login gagal:"
  echo "$RESPONSE"
  exit 1
fi

echo "$TOKEN" > .admin_token
echo "✅ Token admin disimpan di .admin_token"
echo "Token: ${TOKEN:0:32}..."