import _bala, { BalaBala } from 'balabala'
import { load } from 'cheerio';

(async function main() {
  let balabala = new BalaBala("https://docs.rs/v8/latest/v8");

  const hostName = balabala.get_host_name();

  const page_html = await balabala.fetch_html("/");
  // const page_html = await _bala.get_html(hostName, "/v8/0.71.2/v8");

  let html_context = load(page_html);

  const body_context = docs_parse(html_context);
  // const body_html = html_context('body').html();  // 或者其他你想提取的元素

  const links = get_link_all(body_context);
  // console.log("【 links 】==>", links);

  const linkss = [
    'struct.AccessorConfiguration.html',
    'struct.AccessorSignature.html',
    'struct.Allocator.html',
    'struct.Array.html',
    'struct.ArrayBuffer.html',
    'struct.ArrayBufferView.html',
  ]

  const htmlArray = await balabala.fetch_html_promise_all(linkss);
  // console.log("【 htmlArray 】==>", htmlArray);


})()


function docs_parse(html_context) {
  // const pure = html_context('.pure-g').text();
  html_context('svg').remove(); // 删除svg
  html_context('img').remove(); // 删除svg

  return html_context
}

function get_link_all(body_context) {
  // const body_context = load(body);
  const links = []
  body_context('a').each((_i, item) => {
    const url = body_context(item).attr('href');
    links.push(url)
  })

  console.log("【 links 】==>", links);
  return links
}
