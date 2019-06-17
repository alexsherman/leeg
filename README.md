# leeg
ROADMAP and thoughts
https://docs.google.com/document/d/1Adfo9WnFONJDwrYpxAg7a41n_MS_BTF3DU_Y0moHhQ8/edit?usp=sharing
feel free to comment, edit, or add here

## connecting to DB
db is postgres 11, instance is named matches-na, db is named matches

get password from alex and be added as editor to project on GC

log on to google cloud console, get the databse IP from overview page, and whitelist your ip on the connections page

connect without ssl (for now)

## how to run web app
1. run `npm install` in src/javascript/websocket-test and src/javascript/reqme
2. run `cargo build` in src/rust/banana (requires nightly version of rust: `rustup default nightly`)
2.5 make sure `libssl-dev` and `pkg-config` are installed on your machine - necessary for rust request library used to call riot API routes.
3. install redis:
 ```
 wget http://download.redis.io/redis-stable.tar.gz
tar xvzf redis-stable.tar.gz
cd redis-stable
sudo make install //this will add redis-cli and redis-server to your path
 ```
 4. run redis with the conf file in the main directory
 ```
 redis-server redis.conf
 ```
 6. make sure you have a Db_config.toml filled out with the right values in src/rust/banana
 5. start your api. from src/rust/banana. You may need to adjust the paths for the champion files in lib.rs until we figure out a better solution for that:
 ```
 cargo run
 ```
 6. start the test websocket. go to src/javascript/websocket-test and run:
 ```
 node test.js
 ```
 7. start the web app. go to src/javascript/reqme and run:
 ```
 npm start
 ```
 8. Now you can test it out! In the shell that is running the test websocket, you should see a message 'connected'. That means you're ready to broadcast champion picks to the webapp. Type the exact name of any champion in the shell and hit enter. That champ should pop up in the webapp, and it should automatically fire off a request to your api and display the reqs. Keep adding champs this way. You can type 'clear' and hit enter to remove all champs added. 

## how to setup and run the java client listener

The `client-listener` app listens to the LoL client and broadcasts pick/ban data over a local websocket.
For now, it runs on Windows only (this is a limitation of the client API library).

See https://github.com/stirante/lol-client-java-api for the library we're using.

 0. Until merged, you'll need to check out branch `feature/client-listener`
 1. Install a JDK for Java 8 on your Windows side, free & recommended is AdoptOpenJDK: https://adoptopenjdk.net/installation.html#
 2. Install maven on your Windows side: https://maven.apache.org/install.html (be sure to add maven to your PATH)
 3. Open your LoL client and log in
 4. Open `cmd` as Administrator. Check your maven installation with `mvn --version`
 5. Within `leeg`, navigate to `src/java/client-listener`
 6. Run `mvn clean install && mvn exec:java`
 7. The app should start up, tell you the summoner logged in, and try to open a connection to the websocket the Web app is listening on
    
    On startup the client-listener app will print a sample of the JSON format that will be broadcast over the local websocket:
    {"summonerName":"Rawshokwave","summonerTeam":[137,42,11],"opponentTeam":[642,55]}

## future data source for professional league matches 
http://oracleselixir.com/

## game images and assets quick link
https://github.com/CommunityDragon/Docs/blob/master/assets.md
