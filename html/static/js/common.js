const domainRegex = /^(?:(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z][a-zA-Z0-9-]{0,61}[a-zA-Z0-9])(?::\d{1,5})?$/;
const ipv4Regex = /^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/;

const DNS_MAP = new Map([
    ['google', 'Google Dns'],
    ['open', 'Open Dns'],
    ['cloudflare', 'Cloudflare Dns'],
    ['ali', '阿里 Dns'],
    ['114', '114 Dns'],
]);

// 定义 get 方法
function getDnsDesc(key) {
    return DNS_MAP.get(key) || null;
}



function isEmpty(str) {
    if (str === null || str === undefined || str === '') {
        return true;
    }
    return !/\S/.test(str.replace(/^\s+|\s+$/g, ''));
}