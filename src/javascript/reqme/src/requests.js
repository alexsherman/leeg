export function getChampions() {
    return getAsJSON("http://localhost:8000/champions")
}

export function getGlobalRecommendations(sameTeam, oppTeam, roles) {
    const baseUrl = 'http://localhost:8000/globalreq';
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
