**A PoC for Rust API using Rocket with TLS**


**Host configuration to run reqwest integration tests**<br />
    In order for these tests to succeed with the self signed certificate with the cn=api.phonax.com, I created a hosts in the /etc/hosts file<br />
    127.0.0.1	api.phonax.com<br />
    Without this you would get a Warning: tls handshake with 127.0.0.1:xxxxxx failed: tls handshake eof error because 
    Reqwest at standard validates the hostname (as it should!!!)

**CD of certificate and key**<br />
    Deployment of certificates are easily doable in Conitnous Deliver (CD), where we can copy the generated certificate and key in to the Docker container during it's creation.<br />
    By simply copying them to the locations as definied in the Rocket.toml and these files need to be only be readable by the NPA user that runs the service.<br />
<br />
**Docker is your friend**<br />
    Since we create by default a statically linked binary, we do not have to worry about installing Runtime envinments in our Dockerfile and keeping those up to date in order to keep the code running (right .Net and JRE!!!)
    