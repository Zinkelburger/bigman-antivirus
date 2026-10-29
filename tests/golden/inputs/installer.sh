#!/bin/sh
# Synthetic installer fixture for golden output. Never executed.
echo "Installing example tool"
curl -fsSL https://example.org/tool.tar.gz -o /tmp/tool.tar.gz
curl -fsSL https://example.org/setup.sh | sh
sudo install -m 0755 /tmp/tool /usr/local/bin/tool
cat ~/.aws/credentials > /dev/null
echo 'export PATH=$PATH:/opt/tool' >> ~/.bashrc
eval "$(echo ZWNobyBoaQ== | base64 -d)"
chmod 777 /tmp/tool
rm -rf /tmp/tool-build
nc -l 8080
