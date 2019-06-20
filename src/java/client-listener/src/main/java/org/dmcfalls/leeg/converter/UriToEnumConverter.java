package org.dmcfalls.leeg.converter;

import org.dmcfalls.leeg.domain.uri.PickBanUriEnum;

import static org.dmcfalls.leeg.constants.UriConstants.CHAMP_SELECT_BANNABLE_CHAMPIONS_URI;
import static org.dmcfalls.leeg.constants.UriConstants.CHAMP_SELECT_DISABLED_CHAMPIONS_URI;
import static org.dmcfalls.leeg.constants.UriConstants.CHAMP_SELECT_PICKABLE_CHAMPIONS_URI;
import static org.dmcfalls.leeg.constants.UriConstants.CHAMP_SELECT_SESSION_URI;
import static org.dmcfalls.leeg.constants.UriConstants.TEAM_BUILDER_CHAMP_SELECT_SESSION_URI;
import static org.dmcfalls.leeg.constants.UriConstants.TEAM_BUILDER_CHAMP_SELECT_PICKABLE_CHAMPIONS_URI;
import static org.dmcfalls.leeg.constants.UriConstants.TEAM_BUILDER_CHAMP_SELECT_BANNABLE_CHAMPIONS_URI;
import static org.dmcfalls.leeg.constants.UriConstants.TEAM_BUILDER_CHAMP_SELECT_DISABLED_CHAMPIONS_URI;

public final class UriToEnumConverter {

    private UriToEnumConverter() {
        throw new AssertionError("Utility class not meant to be instantiated!");
    }

    public static PickBanUriEnum toPickBanUriEnum(String uri) {
        if (CHAMP_SELECT_SESSION_URI.equals(uri)) {
            return PickBanUriEnum.SESSION;
        } else if (CHAMP_SELECT_PICKABLE_CHAMPIONS_URI.equals(uri)) {
            return PickBanUriEnum.PICKABLE_CHAMPIONS;
        } else if (CHAMP_SELECT_BANNABLE_CHAMPIONS_URI.equals(uri)) {
            return PickBanUriEnum.BANNABLE_CHAMPIONS;
        } else if (CHAMP_SELECT_DISABLED_CHAMPIONS_URI.equals(uri)) {
            return PickBanUriEnum.DISABLED_CHAMPIONS;
        } else if (TEAM_BUILDER_CHAMP_SELECT_SESSION_URI.equals(uri)) {
            return PickBanUriEnum.TEAM_BUILDER_LOBBY_SESSION;
        } else if (TEAM_BUILDER_CHAMP_SELECT_PICKABLE_CHAMPIONS_URI.equals(uri)) {
            return PickBanUriEnum.TEAM_BUILDER_LOBBY_PICKABLE_CHAMPIONS;
        } else if (TEAM_BUILDER_CHAMP_SELECT_BANNABLE_CHAMPIONS_URI.equals(uri)) {
            return PickBanUriEnum.TEAM_BUILDER_LOBBY_BANNABLE_CHAMPIONS;
        } else if (TEAM_BUILDER_CHAMP_SELECT_DISABLED_CHAMPIONS_URI.equals(uri)) {
            return PickBanUriEnum.TEAM_BUILDER_LOBBY_DISABLED_CHAMPIONS;
        }else {
            return PickBanUriEnum.UNKNOWN;
        }
    }

}
