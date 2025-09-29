#!/bin/bash

# Cek apakah token tersedia
if [ ! -f .user_token ]; then
  echo "❌ File .user_token tidak ditemukan. Jalankan dulu 02-login-user.sh"
  exit 1
fi
if [ ! -f .admin_token ]; then
  echo "❌ File .admin_token tidak ditemukan. Jalankan dulu 03-login-admin.sh"
  exit 1
fi

USER_TOKEN=$(cat .user_token)
ADMIN_TOKEN=$(cat .admin_token)

echo "🔓 Menguji akses /user dengan token USER:"
curl -s -H "Authorization: Bearer $USER_TOKEN" http://localhost:3000/user
echo

echo "🔓 Menguji akses /admin dengan token USER (harus 403):"
curl -s -w " [HTTP: %{http_code}]\n" -H "Authorization: Bearer $USER_TOKEN" http://localhost:3000/admin -o /dev/null

echo "👑 Menguji akses /admin dengan token ADMIN:"
curl -s -H "Authorization: Bearer $ADMIN_TOKEN" http://localhost:3000/admin
echo

echo "👑 Menguji akses /user dengan token ADMIN (boleh):"
curl -s -H "Authorization: Bearer $ADMIN_TOKEN" http://localhost:3000/user
echo