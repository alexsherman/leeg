package org.dmcfalls.leeg.utils;

import org.dmcfalls.leeg.exceptions.NotRunningAsAdminException;

public final class AssertUtils {

    private static final String ADMIN_GROUP_CODE = "S-1-5-32-544";

    private AssertUtils() {
        throw new AssertionError("Utility class not meant to be instantiated!");
    }

    /**
     * Asserts that the user of this process has administrator privileges
     * Note: only works on a Windows OS
     * @throws NotRunningAsAdminException if user is not an administrator
     */
    public static void assertRunningAsAdmin() throws NotRunningAsAdminException {
        String[] groups = (new com.sun.security.auth.module.NTSystem()).getGroupIDs();
        for (String group : groups) {
            if (ADMIN_GROUP_CODE.equals(group))
                return;
        }
        throw new NotRunningAsAdminException("This app must be run as administrator!");
    }

}
