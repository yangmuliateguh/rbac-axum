#!/bin/bash
RESPONSE=$(curl -s -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{"username":"user","password":"user123"}')

# Simpan token ke file sementara
TOKEN=$(echo "$RESPONSE" | grep -o '"token":"[^"]*"' | cut -d'"' -f4)
if [ -z "$TOKEN" ] || [ "$TOKEN" = "null" ]; then
  echo "❌ Login gagal:"
  echo "$RESPONSE"
  exit 1
fi

echo "$TOKEN" > .user_token
echo "✅ Token user disimpan di .user_token"
echo "Token: ${TOKEN:0:32}..."