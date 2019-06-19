package org.dmcfalls.leeg;

import com.google.gson.Gson;
import org.dmcfalls.leeg.domain.ChampionSelectSnapshot;
import org.dmcfalls.leeg.exceptions.InitializationException;
import org.dmcfalls.leeg.exceptions.NotRunningAsAdminException;
import org.dmcfalls.leeg.listener.ClientListener;
import org.dmcfalls.leeg.listener.ClientListenerImpl;

import java.util.Arrays;
import java.util.HashSet;

import static org.dmcfalls.leeg.utils.AssertUtils.assertRunningAsAdmin;

/**
 * Opens a listener on the web socket that the LoL client publishes to.
 * Filters out pick/ban events, parses them, and converts them to a useful format,
 * then publishes this data to another local web socket to be consumed by another application or website.
 */
public class ClientListenerApp {

    private static final long SLEEP_INTERVAL_MS = 1000;

    /**
     * Entry point to the client-listener app
     * @param args currently unused
     * @throws InitializationException if an error occurs while initializing the listener web socket
     * @throws NotRunningAsAdminException if this process is not being run with administrative privileges
     */
    public static void main(String[] args) throws InitializationException, NotRunningAsAdminException {
        System.out.println("Welcome to ClientListener!");
        printSampleJson();
        assertRunningAsAdmin();
        ClientListener clientListener = new ClientListenerImpl();
        clientListener.init();
        while(true) {
            try {
                Thread.sleep(SLEEP_INTERVAL_MS);
            } catch (InterruptedException e) {
                System.out.println("ClientListener app interrupted, cleaning up and exiting...");
                break;
            }
        }
        clientListener.close();
        System.out.println("Goodbye!");
    }

    /**
     * Prints a sample of the JSON format that will be broadcast over the local websocket
     */
    private static void printSampleJson() {
        ChampionSelectSnapshot css = new ChampionSelectSnapshot();
        css.setSummonerName("Rawshokwave");
        css.setSummonerTeam(new HashSet<>(Arrays.asList(11, 42, 137)));
        css.setOpponentTeam(new HashSet<>(Arrays.asList(642, 55)));
        System.out.println("Sample champion select data JSON: '" + new Gson().toJson(css) + "'");
    }

}
