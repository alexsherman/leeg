package org.dmcfalls.leeg;

import junit.framework.Test;
import junit.framework.TestCase;
import junit.framework.TestSuite;

/**
 * Unit test for simple ClientListenerApp.
 */
public class ClientListenerAppTest extends TestCase {

    public ClientListenerAppTest(String testName) {
        super(testName);
    }

    public static Test suite() {
        return new TestSuite(ClientListenerAppTest.class);
    }

    public void testApp() {
        assertTrue( true );
    }

}
