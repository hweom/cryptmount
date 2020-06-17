# cryptmount
A utility to mount dm-crypt volumes written in Rust.

## Installation

Clone and build the repo:
```
$ git clone https://github.com/hweom/cryptmount.git
$ cd cryptmount
$ cargo build --release
```

Then copy the binary to `/opt/cryptmount/`
```
$ mkdir /opt/cryptmount
$ cp target/release/cryptmount /opt/cryptmount/
```

Install the `pam_mount` package.

Edit the `/etc/security/pam_mount.conf.xml` to have the following lines:
```
<pam_mount>
  <volume user="userX" fstype="crypt" path="/dev/sdaX" mountpoint="/home/%(USER)"/>
  <mntoptions allow="nosuid,nodev,loop,encryption,fsck,nonempty,allow_root,allow_other" />
  <mntoptions require="nosuid,nodev" />

  <!-- requires ofl from hxtools to be present --> 
  <logout wait="0" hup="no" term="no" kill="no" />

  <mkmountpoint enable="1" remove="true" />

  <cryptmount>/opt/cryptmount/cryptmount open %(VOLUME) %(MNTPT)</cryptmount>
  <cryptumount>/opt/cryptmount/cryptmount close %(VOLUME) %(MNTPT)</cryptumount>
</pam_mount>
```

(Make sure to replace `userX` and `/dev/sdaX` with your values).

Edit the `etc/pam.d/system-login` file and add the following entries:
<pre>
auth       required   pam_tally2.so        onerr=succeed file=/var/log/tallylog
auth       required   pam_shells.so
auth       requisite  pam_nologin.so
auth       include    system-auth
<b>auth       optional   pam_mount.so</b>

account    required   pam_tally2.so 
account    required   pam_access.so
account    required   pam_nologin.so
account    include    system-auth

<b>password   optional   pam_mount.so</b>
password   include    system-auth

session    optional   pam_loginuid.so
session    optional   pam_keyinit.so       force revoke
<b>session [success=1 default=ignore]  pam_succeed_if.so  service = systemd-user quiet
session    optional   pam_mount.so</b>
session    include    system-auth
session    optional   pam_motd.so          motd=/etc/motd
session    optional   pam_mail.so          dir=/var/spool/mail standard quiet
-session   optional   pam_systemd.so
session    required   pam_env.so
</pre>
