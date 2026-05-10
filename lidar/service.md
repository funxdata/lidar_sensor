#### lidar instrll


- vim service

```

sudo nano /etc/systemd/system/lidar.service

```

```

[Unit]
Description=Lidar Service
After=network.target

[Service]
Type=simple
User=dikety
WorkingDirectory=/home/dikety/codes/lidar
ExecStart=/home/dikety/codes/lidar/lidar
Restart=always
RestartSec=3

[Install]
WantedBy=multi-user.target

```


- instrll lidar

```

sudo systemctl daemon-reload

sudo systemctl enable lidar

sudo systemctl start lidar

```

