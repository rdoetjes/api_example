# PoC for Rust API using Rocket with TLS
This is just a proof of concept to see if TLS and mutual TLS can be implemented. And how we would need to deploy the certificates -- every framework has it's own method. And in order to give my client a correct time estimate on implementing this with the whole certificate life cycle management, I started this PoC and decided to share it's development on my youtube channel.

So the database only has a very crued create (no id's etc) and read and delete, I did not even bother to implement an update. The database implementation is more a demo piece for the video, and holds no secrets for us.

## Host configuration to run reqwest integration tests
In order for these tests to succeed with the self signed certificate with the cn=api.phonax.com, I created a hosts in the /etc/hosts file<br />
```
    127.0.0.1	api.phonax.com
```
Without this you would get a Warning: tls handshake with 127.0.0.1:xxxxxx failed: tls handshake eof error because 
Reqwest at standard validates the hostname (as it should!!!)

## CD of certificate and key
Deployment of certificates are easily doable in Conitnous Deliver (CD), where we can copy the generated certificate and key in to the Docker container during it's creation.<br />
   By simply copying them to the locations as definied in the Rocket.toml and these files need to be only be readable by the NPA user that runs the service.<br />

## Docker is your friend
Since we create by default a statically linked binary, we do not have to worry about installing Runtime envinments in our Dockerfile and keeping those up to date in order to keep the code running (right .Net and JRE!!!)
    