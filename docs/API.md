# Recipe Finder API 卡片

> 供前后端讨论的草稿，不代表服务已经实现。
>
> 本稿提案：沿用团队抓取的菜谱字段，保留用量和步骤；第一版先做healthcheck、菜名搜索和食材搜索。耗时、饮食标签需要补充数据后再接入。

## 通用约定

- 后端地址默认 `http://localhost:3000`，允许配置修改。前端可用 Vite 将 `/api` 代理到后端。
- 以下接口均使用 `GET`，无请求体，无需登录。响应为 UTF-8 JSON，`Content-Type: application/json`。
- 中文查询参数用 `URLSearchParams` 或 `curl --data-urlencode` 编码。
- 搜索结果统一为 `{ "items": [...], "total": 数量 }`，暂不分页。

---

## API 卡片 01｜检查服务

| 项目 | 约定 |
| --- | --- |
| 接口 | `GET /api/health` |
| 用途 | 确认 HTTP 服务可访问，不检查数据库等依赖 |
| 参数 | 无；传查询参数返回 `400 INVALID_QUERY` |
| 成功状态 | `200 OK` |

**请求：**

```sh
curl 'http://localhost:3000/api/health'
```

**返回：**

```json
{ "status": "ok" }
```

---

## API 卡片 02｜获取或搜索菜谱

| 项目 | 约定 |
| --- | --- |
| 接口 | `GET /api/recipes` |
| 用途 | 获取全部菜谱，或按菜名、食材查找 |
| 成功状态 | `200 OK`；没有匹配结果也返回 `200` |
| 返回内容 | `items` 是筛选、排序后的完整列表；`total` 等于 `items.length` |

**参数全部可选：**

| 参数 | 含义 | 示例 | 不传时 |
| --- | --- | --- | --- |
| `q` | 菜名关键词，匹配 `title` | `红烧` | 不限制菜名 |
| `ingredients` | 食材名称，前端用英文逗号连接 | `猪腰子,茭白` | 不限制食材 |
| `sort` | 排序方式；第一版只支持 `relevance`（相关性） | `relevance` | 按相关性排序 |

**搜索规则：**

1. 菜名包含关键词即可，例如“红烧”可以找到“红烧肉”。空关键词、空食材列表等同于不限制。
2. 多个食材命中任意一个即可；比较 `ingredients[].name`，不搜索用量。除别名处理外，食材名称需完整相同，例如“肉”不匹配“猪肉”。
3. 同时传菜名和食材时，两类条件都要满足。
4. 默认先排菜名完全相同的，再按命中的不同食材数量从多到少排列，最后按字符串 `id` 升序。无条件时按字符串 `id` 升序，不按数值大小排。

**请求：**

```sh
# 获取全部菜谱
curl 'http://localhost:3000/api/recipes'

# 按菜名搜索
curl --get 'http://localhost:3000/api/recipes' \
  --data-urlencode 'q=爆炒腰花'

# 按食材搜索
curl --get 'http://localhost:3000/api/recipes' \
  --data-urlencode 'ingredients=猪腰子,茭白'
```

**成功返回：** 对应 `q=爆炒腰花`。示例来自团队的抓取数据，选取接口需要的字段，内容未补写。

```json
{
  "items": [
    {
      "id": "560",
      "title": "爆炒腰花",
      "description": "",
      "ingredients": [
        { "name": "猪腰子", "amount": "" },
        { "name": "茭白", "amount": "" },
        { "name": "葱", "amount": "" },
        { "name": "酱油", "amount": "" },
        { "name": "盐", "amount": "" },
        { "name": "料酒", "amount": "" },
        { "name": "淀粉", "amount": "" },
        { "name": "糖", "amount": "" }
      ],
      "steps": [
        { "number": 1, "text": "腰花买一破二，把中间白色的地方去掉，外面的一层膜去掉。用花刀切好，用盐、淀粉、料酒腌制一下。茭白切丝备用", "image_urls": [] },
        { "number": 2, "text": "起油锅放姜先爆炒腰花，放茭白，加酱油、糖、盐等调味", "image_urls": [] },
        { "number": 3, "text": "盛起加葱就可以了", "image_urls": [] }
      ],
      "tips": "",
      "cover_image_urls": [
        "https://i2.chuimg.com/a9ba4f287c8e11e591cde0db5512b208.jpg?imageView2/1/w/640/h/520/q/75/format/jpg"
      ],
      "author": "小月紫",
      "author_url": "https://mip.xiachufang.com/cook/100446/",
      "url": "https://mip.xiachufang.com/recipe/560/"
    }
  ],
  "total": 1
}
```

**没有结果：** 前端显示“没有找到相关菜谱”。

```json
{ "items": [], "total": 0 }
```

<details>
<summary>后端实现补充：参数边界、食材别名和预留功能</summary>

- 参数名区分大小写；未知参数、重复参数、错误的 URL 编码或无效 UTF-8 返回 `400 INVALID_QUERY`。URL 只解码一次，`+` 表示空格，字面加号传 `%2B`。
- `q` 解码后最多 100 个 Unicode 字符（码点）。查询文本和菜名都做 NFKC 标准化（统一全角等兼容字符）、转小写、去掉两端空白、连续空白合并为一个空格。整个 `q` 作为词组匹配，不拆词，不纠正错别字。
- `ingredients` 解码后最多 1000 个码点；先做 NFKC，再按英文逗号、中文逗号或 Unicode 空白分隔。去掉空项后最多 20 项，每项按上述规则标准化后最多 50 个码点。
- 查询和数据中的食材名称都按上述规则标准化，并替换别名：`土豆 → 马铃薯`、`西红柿 → 番茄`。查询项在别名替换后去重，同一食材只计一次命中；响应保留抓取原文。
- 未传菜名或食材时，对应排序分值为 0。命中食材多不代表已备齐全部食材；盐、油也参与匹配。
- `sort` 只允许 `relevance`、`time_asc`、`time_desc`，不去空白、不转换大小写，`sort=` 非法。当前没有耗时数据，后两项暂返回 `400 UNSUPPORTED_PARAMETER`。
- 预留 `dietary_tags`：枚举为 `vegetarian`（素食）、`vegan`（纯素）、`gluten_free`（无麸质）、`keto`（生酮）。解码后最多 100 个码点，用英文逗号分隔，去掉两端空白、空项并去重，最多 4 项。空列表不限制；合法非空列表暂返回 `400 UNSUPPORTED_PARAMETER`；未知标签返回 `400 INVALID_QUERY`。
- 先校验所有参数，再判断功能是否支持，然后筛选、排序。若某阶段尚未实现食材搜索，合法非空食材条件也返回 `UNSUPPORTED_PARAMETER`，不能忽略条件。

</details>

---

## 数据卡片｜每道菜谱的字段

`items` 中的每个对象都包含以下字段，食材和步骤也必须包含表中的子字段。

| 字段 | 类型 | 含义 / 前端用法 |
| --- | --- | --- |
| `id` | 字符串 | 沿用抓取 ID，如 `"560"`；作为稳定标识和 React key，不转数字 |
| `title` | 字符串 | 菜名 |
| `description` | 字符串 | 简介；空值隐藏，较长时由前端截断展示 |
| `ingredients` | 对象数组 | 每项含 `name`（食材名）、`amount`（用量），均为字符串 |
| `steps` | 对象数组 | 每项含 `number`（从 1 开始的整数）、`text`（字符串）、`image_urls`（图片地址字符串数组），按 `number` 升序返回 |
| `tips` | 字符串 | 小贴士；空值隐藏 |
| `cover_image_urls` | 字符串数组 | 封面；优先显示第一张，没有图片或加载失败时用占位图 |
| `author` | 字符串 | 原作者 |
| `author_url` | 字符串 | 原作者主页 |
| `url` | 字符串 | 原菜谱页面 |

**空值处理：** 没有文字用 `""`，没有列表内容用 `[]`，不省略字段，也不用 `null`。`amount: ""` 表示原文未给出用量，可显示“用量未提供”，不要自动改成“适量”。步骤没有图片时正常显示文字。

**抓取数据接入：**

- 团队的 `recipes.json` 顶层是数组，共 50 条，目前由团队另行提供，未收录进仓库。后端选取上表字段，筛选、排序后包装成 `{ "items": [...], "total": 数量 }`。
- `stats_text`、`created_at`、`fetched_at`、`source_sha256`、`quality` 保留在原始数据中，第一版接口暂不返回。`quality.complete` 不代表用量和步骤图片都有内容。
- 原始数据没有饮食标签和准备 / 烹饪 / 总耗时。第一版不返回这些字段，不填 `0`，不从菜名或步骤中猜测；补齐数据并确认规则后再开放相关功能。
- 第一版返回完整菜谱对象，前端可直接展开食材和步骤，暂不另设详情接口，也不涉及账号或数据库接口。

**与旧稿的变化：** `name` 改为 `title`；`ingredients` 从名称数组改为带用量的对象数组；`image_url` 改为 `cover_image_urls`；增加步骤、小贴士和来源信息；暂不返回耗时、饮食标签字段。这些变化需前后端一起确认。

---

## 错误卡片｜请求失败

**例：** 请求 `sort=fastest` 时，返回 `400` 和以下 JSON。

```json
{
  "error": {
    "code": "INVALID_QUERY",
    "message": "查询参数不合法。",
    "details": [
      { "field": "sort", "message": "必须为 relevance、time_asc 或 time_desc。" }
    ]
  }
}
```

| HTTP 状态 | `error.code` | 含义 |
| --- | --- | --- |
| `400` | `INVALID_QUERY` | 参数写错、超长、重复或编码错误 |
| `400` | `UNSUPPORTED_PARAMETER` | 参数合法，但当前阶段尚未支持，例如耗时排序 |
| `404` | `NOT_FOUND` | `/api` 下的路径不存在；不用于搜索无结果 |
| `405` | `METHOD_NOT_ALLOWED` | 已知接口使用了不支持的方法 |
| `500` | `INTERNAL_ERROR` | 服务端异常 |

前端用 `code` 判断错误类型，`message` 用于展示。所有错误都有 `details`：`400` 至少一项，`field` 填参数名，整段编码错误填 `query`；其他状态使用空数组。多个参数写错时，可以只报告一个。

断网、超时或收到非 JSON 响应时，前端显示“请求失败，请重试”。服务端异常响应不要暴露堆栈、文件路径或配置秘密。

<details>
<summary>联调补充：请求状态、HEAD 和跨域</summary>

- 前端分别处理加载中、成功、空结果、失败；新搜索开始后取消旧请求或忽略旧响应，避免旧结果覆盖新结果。
- `HEAD` 返回对应 `GET` 的状态和响应头，无正文。已知接口支持 `GET, HEAD, OPTIONS`，`405` 响应带 `Allow: GET, HEAD, OPTIONS`。
- 直连不同端口时，后端允许实际使用的前端 origin，以及 `GET, HEAD, OPTIONS` 和 `Accept` 请求头；成功的跨域预检返回 `204`，无正文。无凭证请求；`localhost` 与 `127.0.0.1` 是不同 origin。
- 后端的参数解析错误也统一使用本页的 JSON 格式。

</details>

## Review

- [ ] 确认沿用抓取字段，以及食材、步骤、图片的结构和空值处理。
- [ ] 确认第一版范围：菜名搜索、食材搜索、相关性排序；耗时和饮食标签暂缓。
- [ ] 用同一份数据联调：无参数返回全部 50 条；搜“爆炒腰花”返回 `id: "560"`；无匹配返回空列表；非法参数返回 `400`。
- [ ] 确认本地端口、代理或跨域设置，并实际发送请求验证。

确认项待队员 review 后勾选。当前仓库尚无配套 OpenAPI、独立 mock 文件和契约校验脚本，联调以本页的字段、规则和 JSON 示例为准。
