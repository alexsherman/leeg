package org.dmcfalls.leeg.constants;

/**
 * Not to be confused with Yuri constants
 */
public final class UriConstants {

    private static final String CHAMP_SELECT_URI_PREFIX = "/lol-champ-select/v1/";
    private static final String TEAM_BUILDER_CHAMP_SELECT_URI_PREFIX = "/lol-lobby-team-builder/champ-select/v1/";

    private static final String SESSION = "session";
    private static final String PICKABLE_CHAMPIONS = "pickable-champions";
    private static final String BANNABLE_CHAMPIONS = "bannable-champions";
    private static final String DISABLED_CHAMPIONS = "disabled-champions";

    public static final String CHAMP_SELECT_SESSION_URI = CHAMP_SELECT_URI_PREFIX + SESSION;
    public static final String CHAMP_SELECT_PICKABLE_CHAMPIONS_URI = CHAMP_SELECT_URI_PREFIX + PICKABLE_CHAMPIONS;
    public static final String CHAMP_SELECT_BANNABLE_CHAMPIONS_URI = CHAMP_SELECT_URI_PREFIX + BANNABLE_CHAMPIONS;
    public static final String CHAMP_SELECT_DISABLED_CHAMPIONS_URI = CHAMP_SELECT_URI_PREFIX + DISABLED_CHAMPIONS;

    public static final String TEAM_BUILDER_CHAMP_SELECT_SESSION_URI = TEAM_BUILDER_CHAMP_SELECT_URI_PREFIX + SESSION;
    public static final String TEAM_BUILDER_CHAMP_SELECT_PICKABLE_CHAMPIONS_URI = TEAM_BUILDER_CHAMP_SELECT_URI_PREFIX + PICKABLE_CHAMPIONS;
    public static final String TEAM_BUILDER_CHAMP_SELECT_BANNABLE_CHAMPIONS_URI = TEAM_BUILDER_CHAMP_SELECT_URI_PREFIX + BANNABLE_CHAMPIONS;
    public static final String TEAM_BUILDER_CHAMP_SELECT_DISABLED_CHAMPIONS_URI = TEAM_BUILDER_CHAMP_SELECT_URI_PREFIX + DISABLED_CHAMPIONS;

    private UriConstants() {
        throw new AssertionError("Not meant to be instantiated!");
    }

}
