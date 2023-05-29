import bala, { BalaBala } from 'balabala'
import { load } from 'cheerio';
import fs from 'fs'
import path from 'path'

(async function main() {
  let balabala = new BalaBala("https://docs.rs/v8/latest/v8/");

  const hostName = balabala.get_host_name();

  // 从url获取首页内容
  const page = await balabala.fetch_html("");
  // 过滤首页中的内容：svg,img
  const body_context = docs_parse(page);
  // 首页HTML写入到文件
  make_resource("resource/v8/index.html", body_context.html());
  // 获取首页中的所有链接
  let linkss = get_link_all(body_context)

  // linkss = [
  //   'struct.AccessorConfiguration.html',
  //   'struct.AccessorSignature.html',
  //   "fast_api/index.html",
  //   "icu/index.html"
  // ]

  await balabala.fetch_html_all_unordered(linkss, (currentUrl, html) => {
    // 过滤页面中的内容：svg,img
    const body_context = docs_parse(html);
    const make = make_resource(`resource/v8/${currentUrl}`, body_context.html());
  }, doesFileExist);
  // console.log("【 htmlArray 】==>", htmlArray);

})()


function docs_parse(page) {
  // 使用cheerio从获取的HTML 字符串加载首页内容
  let html_context = load(page);
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

    // 保留本站链接
    if (!bala.validate_router(url)) {
      links.push(url)
    }
  })

  return links
}

function make_resource(resourceName, content = '') {
  try {
    const pathParsed = path.parse(resourceName)
    fs.mkdirSync(pathParsed.dir, { recursive: true });

    let filePath = path.join(pathParsed.dir, pathParsed.base)
    fs.writeFileSync(filePath, content, 'utf8');

    // console.log('文件创建成功：', filePath);
  } catch (e) {
    console.error('Error:', error.message);
    // 终止程序
    process.exit(1);
  }
}

function doesFileExist(fileName) {
  try {
    // fs.existsSync(fileName)
    fs.accessSync(fileName, fs.constants.R_OK | fs.constants.W_OK);
    return true;
  } catch (err) {
    return false;
  }
}


// 统计文件格式
// ls -l v8/ | grep ^- | wc -l
