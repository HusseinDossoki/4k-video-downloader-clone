const filters = {
  truncate(text, length, clamp) {
    clamp = clamp || '...';
    let node = document.createElement('div');
    node.innerHTML = text;
    let content = node.textContent;
    return content.length > length ? content.slice(0, length) + clamp : content;
  },
  formatTime(seconds) {
    if(!seconds) return;
    // If number of seconds are less than 3600, you can remove hours part and format the string in minutes and seconds.
    if (seconds < 3600) return new Date(seconds * 1000).toISOString().substr(14, 5);
    return new Date(seconds * 1000).toISOString().substr(11, 8);
  },
  formatSize(sizeInbytes, si = true, dp = 1) {
    if(!sizeInbytes) return;
    const thresh = si ? 1000 : 1024;

    if (Math.abs(sizeInbytes) < thresh) {
      return sizeInbytes + ' B';
    }

    const units = si
      ? ['kB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB']
      : ['KiB', 'MiB', 'GiB', 'TiB', 'PiB', 'EiB', 'ZiB', 'YiB'];
    let u = -1;
    const r = 10 ** dp;

    do {
      sizeInbytes /= thresh;
      ++u;
    } while (Math.round(Math.abs(sizeInbytes) * r) / r >= thresh && u < units.length - 1);


    return sizeInbytes.toFixed(dp) + ' ' + units[u];
  },
  humanize(str) {
    if(!str) return;
    return str
        .replace(/^[\s_]+|[\s_]+$/g, '')
        .replace(/[_\s]+/g, ' ')
        .replace(/^[a-z]/, function(m) { return m.toUpperCase(); });
  }
}

export default filters;
