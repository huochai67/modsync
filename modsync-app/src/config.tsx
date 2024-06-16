type config = {
    darkmode: boolean
}

export function setConfig(config: config) {
    localStorage.setItem("modsync-config", JSON.stringify(config));
}

export function getConfig() {
    let configjson = localStorage.getItem("modsync-config");
    if (configjson) {
        let retconfig: config = JSON.parse(configjson);
        return retconfig;
    }
    let defconfig: config = { darkmode: false };
    setConfig(defconfig);
    return defconfig
}