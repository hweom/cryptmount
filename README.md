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
