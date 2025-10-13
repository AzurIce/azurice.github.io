# Macos launchd

```bash
➜  ~ ps -ef | awk 'NR==1 || /launchd/'
  UID   PID  PPID   C STIME   TTY           TIME CMD
    0     1     0   0  5:27PM ??         0:34.04 /sbin/launchd
```

*launchd* 是 MacOS 中所有进程的父进程，可以通过 `launchctl` 命令来和它交互。比如停止 `org.nixos.skhd` “服务”：

```bash
launchctl stop org.nixos.skhd
```

所谓“服务”有两种：*Daemon* 和 *Agent*，分别是系统级的和用户级的，他们都被定义为一个 `.plist` 文件（以用户定义的 `org.nixos.skhd` 服务为例）：

```bash
➜  ~ cat ~/Library/LaunchAgents/org.nixos.skhd.plist
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple Computer//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
        <key>EnvironmentVariables</key>
        <dict>
                <key>PATH</key>
                <string>$HOME/.nix-profile/bin:/etc/profiles/per-user/$USER/bin:/run/current-system/sw/bin:/nix/var/nix/profiles/default/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin</string>
        </dict>
        <key>KeepAlive</key>
        <true/>
        <key>Label</key>
        <string>org.nixos.skhd</string>
        <key>ProcessType</key>
        <string>Interactive</string>
        <key>ProgramArguments</key>
        <array>
                <string>/nix/store/1ksgc8w3gr6l0zhylr57xyxy1347miad-skhd-0.3.9/bin/skhd</string>
                <string>-c</string>
                <string>/etc/skhdrc</string>
        </array>
</dict>
</plist>%
```

除了 `~/LIbrary/LaunchAgents` 目录外，`/` 和 `/System` 下也有对应的 `LaunchDaemons` 和 `LaunchAgents` 文件夹，分别存储了管理员和苹果提供的 *Daemon* 和 *Agent*。

## nix

- nix-darwin
    - `launchd.user.agents`：对应 `~/Library/LaunchAgents`
    - `launchd.agents`：对应 `/Library/LaunchAgents`
- nix-darwin Home Manager
    - `launchd.agents`：对应 `~/Library/LaunchAgents`

`lauunchd.user.agents.<name>.serviceConfig` 有一些服务的设置选项：

