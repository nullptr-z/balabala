import wasm from 'balabala'
import { load } from 'cheerio';

(async function main() {
  const html = await wasm.greet();
  docs_parse(html);
})()


function docs_parse(html) {
  const html_context = load(html);
  // const pure = html_context('.pure-g').text();
  html_context('svg').remove(); // 删除svg
  html_context('img').remove(); // 删除svg

  // 输出到 stdout
  const body = html_context('body').html();  // 或者其他你想提取的元素
  console.log(body);
}
