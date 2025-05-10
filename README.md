# rs-daemon-spike

## Service

### Setting

`/etc/systemd/system/rs-daemon-spike.srvice`

```
[Unit]
Description=プログラムの説明
After=network.target

[Service]
Type=simple
ExecStart=/path/to/rs-daemon-spike
Restart=on-failure
User=実行するユーザー名

[Install]
WantedBy=multi-user.target
```

### Start & Stop

```bash
# 設定ファイルを読み込む
systemctl daemon-reload

# サービスを有効にする
systemctl enable rs-daemon-spike

# サービスを起動する
systemctl start rs-daemon-spike
```

```bash
# 状態確認
systemctl status rs-daemon-spike

# 停止
systemctl stop rs-daemon-spike

# 再起動
systemctl restart rs-daemon-spike
```