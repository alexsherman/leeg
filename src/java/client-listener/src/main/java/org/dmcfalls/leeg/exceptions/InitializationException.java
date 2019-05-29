package org.dmcfalls.leeg.exceptions;

/**
 * Exception thrown during initialization of a ClientListener
 */
public class InitializationException extends RuntimeException {

    public InitializationException(Exception e) {
        super(e);
    }

    public InitializationException(String str) {
        super(str);
    }

}
