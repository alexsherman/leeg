const API_HOST = "https://leeg-240122.appspot.com"

export function getChampions() {
    return getAsJSON(API_HOST + "/champions")
}

export function getGlobalRecommendations(sameTeam, oppTeam, roles) {
    const baseUrl = API_HOST + '/globalreq';
    let params = '?';
    if (sameTeam.length > 0) {
        params += 'team=' + sameTeam.join(',');
    }
    if (oppTeam.length > 0) {
        params += '&opp=' + oppTeam.join(',');
    }
    if (roles.length) {
        params += "&roles=" + roles.join(",");
    }

    return getAsJSON(baseUrl + params);
}

function getAsJSON(url) {
    return fetch(url, {
        method: "GET",
        mode: "cors",
        headers: {
            "Accept": "application/json"
        }
    }).then(resp => {
        return resp.json();
    });
}
