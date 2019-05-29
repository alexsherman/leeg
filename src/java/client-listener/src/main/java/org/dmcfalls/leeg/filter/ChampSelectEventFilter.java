package org.dmcfalls.leeg.filter;

import com.stirante.lolclient.ClientWebSocket;

import java.util.Arrays;
import java.util.List;

import static org.dmcfalls.leeg.constants.UriConstants.CHAMP_SELECT_BANNABLE_CHAMPIONS_URI;
import static org.dmcfalls.leeg.constants.UriConstants.CHAMP_SELECT_DISABLED_CHAMPIONS_URI;
import static org.dmcfalls.leeg.constants.UriConstants.CHAMP_SELECT_PICKABLE_CHAMPIONS_URI;
import static org.dmcfalls.leeg.constants.UriConstants.CHAMP_SELECT_SESSION_URI;

/**
 * Only allows events associated with champion select and containing pick/ban data
 */
public class ChampSelectEventFilter implements ClientEventFilter {

    private static final List<String> ALLOWED_URIS
            = Arrays.asList(
                CHAMP_SELECT_SESSION_URI,
                CHAMP_SELECT_PICKABLE_CHAMPIONS_URI,
                CHAMP_SELECT_BANNABLE_CHAMPIONS_URI,
                CHAMP_SELECT_DISABLED_CHAMPIONS_URI
    );

    @Override
    public boolean allow(ClientWebSocket.Event event) {
        return ALLOWED_URIS.contains(event.getUri());
    }

}
