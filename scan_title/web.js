const urllib = require('url');
const rq = require('request-promise');
const cheerio = require('cheerio');
const path = require('path');

function removeSpace(str) {
  return str.replace(/\s/g, '');
}

function jumpUrl(baseUrl, metaUrl) {
  const baseUri = urllib.parse(removeSpace(baseUrl));
  const metaUri = urllib.parse(removeSpace(metaUrl));
  // url with hostname
  if (metaUri.hostname) {
    return metaUrl;
  }
  // absolute path
  if (metaUri.path[0] === '/') {
    return `${baseUri.protocol}//${baseUri.host}${metaUrl}`;
  }
  // relative path
  let realPath = '';
  if (baseUri.path[baseUri.path.length - 1] === '/') {
    realPath = path.normalize(`/${baseUri.path}${metaUrl}`);
  } else {
    realPath = path.normalize(`/${baseUri.path}/../${metaUrl}`);
  }
  return `${baseUri.protocol}//${baseUri.host}${realPath}`;
}

function jumpMeta(baseUrl, metaArray) {
  if (!metaArray || metaArray.length === 0) {
    return '';
  }
  for (const idx = 0; idx < metaArray.length; ++idx) {
    const meta = metaArray[idx];
    if (meta.attribs['http-equiv'] === 'refresh') {
      const content = meta.attribs['content'];
      const match = /^\s*0\s*;\s*url\s*=\s*(.+)$/.exec(content);
      if (match) {
        return jumpUrl(baseUrl, match[1]);
      }
    }
  }
  return '';
}

function jumpLocation(baseUrl, html) {
  if (html.length > 1024 * 10) {
    return '';
  }
  const locationMatch = /location\s*\.\s*href\s*=\s*[\'\"]([^\'\"])[\'\"]/.exec(html);
  if (!locationMatch) {
    return '';
  }
  return jumpUrl(baseUrl, locationMatch[1]);
}

async function loadPage(url, depth = 1) {
  if (depth > 3) {
    return '';
  }

  try {
    const resp = await rq({
      method: 'GET',
      url,
      followAllRedirects: true,
      resolveWithFullResponse: true,
    });

    const { uri } = resp.request;
    const { href } = uri;
    const html = resp.body;
    const dom = cheerio.load(resp.body);

    const title = dom('title').first().text();
    if (title) {
      return { title, href, html };
    }

    const h1 = dom('h1').first().text();
    if (h1) {
      return { title: h1, href, html };
    }

    const metaUrl = jumpMeta(href, dom('meta'));
    if (metaUrl) {
      return await loadPage(metaUrl, depth + 1);
    }

    const locationUrl = jumpLocation(href, html);
    if (locationUrl) {
      return await loadPage(locationUrl, depth + 1);
    }

  } catch (err) {
    console.log(err);
  }

  return '';
};

loadPage('http://www.infoq.com')
  .then((result) => {
    console.log(result.href);
    console.log(result.title);
  });
