package org.dmcfalls.leeg;

import org.dmcfalls.leeg.exceptions.InitializationException;
import org.dmcfalls.leeg.exceptions.NotRunningAsAdminException;
import org.dmcfalls.leeg.listener.ClientListener;
import org.dmcfalls.leeg.listener.ClientListenerImpl;

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

}
