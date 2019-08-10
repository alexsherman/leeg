## how to run web app
1. run `npm install` in src/javascript/websocket-test and src/javascript/reqme
2. run `cargo build` in src/rust/banana (requires nightly version of rust: `rustup default nightly`)
3 make sure `libssl-dev` and `pkg-config` are installed on your machine - necessary for rust request library used to call riot API routes.
4. install redis:
 ```
 wget http://download.redis.io/redis-stable.tar.gz
tar xvzf redis-stable.tar.gz
cd redis-stable
sudo make install //this will add redis-cli and redis-server to your path
 ```
 5. run redis with the conf file in the main directory
 ```
 redis-server redis.conf
 ```
 6. make sure you have a Db_config.toml filled out with the right values in src/rust/banana
 7. start your api from src/rust/banana. 
 ```
 cargo run
 ```
 8. start the web app. go to src/javascript/reqme and run:
 ```
 npm start
 ```

## how to setup and run the java client listener

The `client-listener` app listens to the LoL client and broadcasts pick/ban data over a local websocket.
For now, it runs on Windows only (this is a limitation of the client API library).

See https://github.com/stirante/lol-client-java-api for the library we're using.

 1. Install a JDK for Java 8 on your Windows side, free & recommended is AdoptOpenJDK: https://adoptopenjdk.net/installation.html#
 2. Install maven on your Windows side: https://maven.apache.org/install.html (be sure to add maven to your PATH)
 3. Open your LoL client and log in
 4. Open `cmd` as Administrator. Check your maven installation with `mvn --version`
 5. Within `leeg`, navigate to `src/java/client-listener`
 6. Run `mvn clean install && mvn exec:java`
 7. The app should start up, tell you the summoner logged in, and try to open a connection to the websocket the Web app is listening on
    
    On startup the client-listener app will print a sample of the JSON format that will be broadcast over the local websocket:
    {"summonerName":"Rawshokwave","summonerTeam":[137,42,11],"opponentTeam":[642,55]}
