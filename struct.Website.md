Website in spider::website - Rustif(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-6b053e98.ttf.woff2,FiraSans-Regular-0fe48ade.woff2,FiraSans-Medium-e1aa3f0a.woff2,SourceCodePro-Regular-8badfe75.ttf.woff2,SourceCodePro-Semibold-aa29a496.ttf.woff2".split(",").map(f=>\`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">\`).join(""))

[spider](../../spider/index.html)2.33.1
---------------------------------------

[Website](#)
------------

### [Fields](#fields)

*   [configuration](#structfield.configuration "configuration")
*   [crawl\_id](#structfield.crawl_id "crawl_id")
*   [on\_link\_find\_callback](#structfield.on_link_find_callback "on_link_find_callback")
*   [on\_should\_crawl\_callback](#structfield.on_should_crawl_callback "on_should_crawl_callback")

### [Methods](#implementations)

*   [build](#method.build "build")
*   [clear](#method.clear "clear")
*   [clear\_all](#method.clear_all "clear_all")
*   [configure\_http\_client](#method.configure_http_client "configure_http_client")
*   [configure\_robots\_parser](#method.configure_robots_parser "configure_robots_parser")
*   [configure\_setup](#method.configure_setup "configure_setup")
*   [configure\_setup\_norobots](#method.configure_setup_norobots "configure_setup_norobots")
*   [crawl](#method.crawl "crawl")
*   [crawl\_raw](#method.crawl_raw "crawl_raw")
*   [crawl\_raw\_send](#method.crawl_raw_send "crawl_raw_send")
*   [crawl\_sitemap](#method.crawl_sitemap "crawl_sitemap")
*   [crawl\_smart](#method.crawl_smart "crawl_smart")
*   [drain\_extra\_links](#method.drain_extra_links "drain_extra_links")
*   [drain\_links](#method.drain_links "drain_links")
*   [drain\_signatures](#method.drain_signatures "drain_signatures")
*   [get\_absolute\_path](#method.get_absolute_path "get_absolute_path")
*   [get\_all\_links\_visited](#method.get_all_links_visited "get_all_links_visited")
*   [get\_client](#method.get_client "get_client")
*   [get\_crawl\_id](#method.get_crawl_id "get_crawl_id")
*   [get\_delay](#method.get_delay "get_delay")
*   [get\_initial\_status\_code](#method.get_initial_status_code "get_initial_status_code")
*   [get\_links](#method.get_links "get_links")
*   [get\_links\_disk](#method.get_links_disk "get_links_disk")
*   [get\_pages](#method.get_pages "get_pages")
*   [get\_size](#method.get_size "get_size")
*   [get\_status](#method.get_status "get_status")
*   [get\_url](#method.get_url "get_url")
*   [get\_url\_parsed](#method.get_url_parsed "get_url_parsed")
*   [is\_allowed](#method.is_allowed "is_allowed")
*   [is\_allowed\_budgetless](#method.is_allowed_budgetless "is_allowed_budgetless")
*   [is\_allowed\_default](#method.is_allowed_default "is_allowed_default")
*   [is\_allowed\_robots](#method.is_allowed_robots "is_allowed_robots")
*   [new](#method.new "new")
*   [persist\_links](#method.persist_links "persist_links")
*   [queue](#method.queue "queue")
*   [reset\_status](#method.reset_status "reset_status")
*   [scrape](#method.scrape "scrape")
*   [scrape\_raw](#method.scrape_raw "scrape_raw")
*   [scrape\_sitemap](#method.scrape_sitemap "scrape_sitemap")
*   [scrape\_smart](#method.scrape_smart "scrape_smart")
*   [set\_crawl\_budget](#method.set_crawl_budget "set_crawl_budget")
*   [set\_extra\_links](#method.set_extra_links "set_extra_links")
*   [set\_http\_client](#method.set_http_client "set_http_client")
*   [set\_url](#method.set_url "set_url")
*   [set\_url\_only](#method.set_url_only "set_url_only")
*   [setup\_disk](#method.setup_disk "setup_disk")
*   [sitemap\_crawl](#method.sitemap_crawl "sitemap_crawl")
*   [size](#method.size "size")
*   [stop](#method.stop "stop")
*   [subscribe](#method.subscribe "subscribe")
*   [subscribe\_guard](#method.subscribe_guard "subscribe_guard")
*   [target\_id](#method.target_id "target_id")
*   [unsubscribe](#method.unsubscribe "unsubscribe")
*   [with\_auth\_challenge\_response](#method.with_auth_challenge_response "with_auth_challenge_response")
*   [with\_automation\_scripts](#method.with_automation_scripts "with_automation_scripts")
*   [with\_blacklist\_url](#method.with_blacklist_url "with_blacklist_url")
*   [with\_block\_assets](#method.with_block_assets "with_block_assets")
*   [with\_budget](#method.with_budget "with_budget")
*   [with\_caching](#method.with_caching "with_caching")
*   [with\_chrome\_connection](#method.with_chrome_connection "with_chrome_connection")
*   [with\_chrome\_intercept](#method.with_chrome_intercept "with_chrome_intercept")
*   [with\_concurrency\_limit](#method.with_concurrency_limit "with_concurrency_limit")
*   [with\_config](#method.with_config "with_config")
*   [with\_cookies](#method.with_cookies "with_cookies")
*   [with\_crawl\_id](#method.with_crawl_id "with_crawl_id")
*   [with\_cron](#method.with_cron "with_cron")
*   [with\_danger\_accept\_invalid\_certs](#method.with_danger_accept_invalid_certs "with_danger_accept_invalid_certs")
*   [with\_delay](#method.with_delay "with_delay")
*   [with\_depth](#method.with_depth "with_depth")
*   [with\_evaluate\_on\_new\_document](#method.with_evaluate_on_new_document "with_evaluate_on_new_document")
*   [with\_execution\_scripts](#method.with_execution_scripts "with_execution_scripts")
*   [with\_external\_domains](#method.with_external_domains "with_external_domains")
*   [with\_fingerprint](#method.with_fingerprint "with_fingerprint")
*   [with\_full\_resources](#method.with_full_resources "with_full_resources")
*   [with\_headers](#method.with_headers "with_headers")
*   [with\_http2\_prior\_knowledge](#method.with_http2_prior_knowledge "with_http2_prior_knowledge")
*   [with\_ignore\_sitemap](#method.with_ignore_sitemap "with_ignore_sitemap")
*   [with\_limit](#method.with_limit "with_limit")
*   [with\_locale](#method.with_locale "with_locale")
*   [with\_no\_control\_thread](#method.with_no_control_thread "with_no_control_thread")
*   [with\_normalize](#method.with_normalize "with_normalize")
*   [with\_on\_link\_find\_callback](#method.with_on_link_find_callback "with_on_link_find_callback")
*   [with\_on\_should\_crawl\_callback](#method.with_on_should_crawl_callback "with_on_should_crawl_callback")
*   [with\_openai](#method.with_openai "with_openai")
*   [with\_preserve\_host\_header](#method.with_preserve_host_header "with_preserve_host_header")
*   [with\_proxies](#method.with_proxies "with_proxies")
*   [with\_proxies\_direct](#method.with_proxies_direct "with_proxies_direct")
*   [with\_redirect\_limit](#method.with_redirect_limit "with_redirect_limit")
*   [with\_redirect\_policy](#method.with_redirect_policy "with_redirect_policy")
*   [with\_request\_timeout](#method.with_request_timeout "with_request_timeout")
*   [with\_respect\_robots\_txt](#method.with_respect_robots_txt "with_respect_robots_txt")
*   [with\_retry](#method.with_retry "with_retry")
*   [with\_return\_page\_links](#method.with_return_page_links "with_return_page_links")
*   [with\_screenshot](#method.with_screenshot "with_screenshot")
*   [with\_service\_worker\_enabled](#method.with_service_worker_enabled "with_service_worker_enabled")
*   [with\_shared\_queue](#method.with_shared_queue "with_shared_queue")
*   [with\_sitemap](#method.with_sitemap "with_sitemap")
*   [with\_stealth](#method.with_stealth "with_stealth")
*   [with\_subdomains](#method.with_subdomains "with_subdomains")
*   [with\_timezone\_id](#method.with_timezone_id "with_timezone_id")
*   [with\_tld](#method.with_tld "with_tld")
*   [with\_user\_agent](#method.with_user_agent "with_user_agent")
*   [with\_viewport](#method.with_viewport "with_viewport")
*   [with\_wait\_for\_delay](#method.with_wait_for_delay "with_wait_for_delay")
*   [with\_wait\_for\_idle\_dom](#method.with_wait_for_idle_dom "with_wait_for_idle_dom")
*   [with\_wait\_for\_idle\_network](#method.with_wait_for_idle_network "with_wait_for_idle_network")
*   [with\_wait\_for\_selector](#method.with_wait_for_selector "with_wait_for_selector")
*   [with\_whitelist\_url](#method.with_whitelist_url "with_whitelist_url")

### [Trait Implementations](#trait-implementations)

*   [Clone](#impl-Clone-for-Website "Clone")
*   [Debug](#impl-Debug-for-Website "Debug")
*   [Default](#impl-Default-for-Website "Default")
*   [Display](#impl-Display-for-Website "Display")
*   [Error](#impl-Error-for-Website "Error")

### [Auto Trait Implementations](#synthetic-implementations)

*   [!Freeze](#impl-Freeze-for-Website "!Freeze")
*   [!RefUnwindSafe](#impl-RefUnwindSafe-for-Website "!RefUnwindSafe")
*   [!UnwindSafe](#impl-UnwindSafe-for-Website "!UnwindSafe")
*   [Send](#impl-Send-for-Website "Send")
*   [Sync](#impl-Sync-for-Website "Sync")
*   [Unpin](#impl-Unpin-for-Website "Unpin")

### [Blanket Implementations](#blanket-implementations)

*   [Any](#impl-Any-for-T "Any")
*   [Borrow<T>](#impl-Borrow%3CT%3E-for-T "Borrow<T>")
*   [BorrowMut<T>](#impl-BorrowMut%3CT%3E-for-T "BorrowMut<T>")
*   [CloneToUninit](#impl-CloneToUninit-for-T "CloneToUninit")
*   [ErasedDestructor](#impl-ErasedDestructor-for-T "ErasedDestructor")
*   [From<T>](#impl-From%3CT%3E-for-T "From<T>")
*   [Instrument](#impl-Instrument-for-T "Instrument")
*   [Into<U>](#impl-Into%3CU%3E-for-T "Into<U>")
*   [IntoEither](#impl-IntoEither-for-T "IntoEither")
*   [MaybeSendSync](#impl-MaybeSendSync-for-T "MaybeSendSync")
*   [Same](#impl-Same-for-T "Same")
*   [ToCompactString](#impl-ToCompactString-for-T "ToCompactString")
*   [ToOwned](#impl-ToOwned-for-T "ToOwned")
*   [ToString](#impl-ToString-for-T "ToString")
*   [TryFrom<U>](#impl-TryFrom%3CU%3E-for-T "TryFrom<U>")
*   [TryInto<U>](#impl-TryInto%3CU%3E-for-T "TryInto<U>")
*   [WithSubscriber](#impl-WithSubscriber-for-T "WithSubscriber")

[In spider::website](index.html)
--------------------------------

[spider](../index.html)::[website](index.html)

Struct WebsiteCopy item path
============================

[Source](../../src/spider/website.rs.html#257-301)

    pub struct Website {
        pub configuration: Box<Configuration>,
        pub on_link_find_callback: Option<fn(_: CaseInsensitiveString, _: Option<String>) -> (CaseInsensitiveString, Option<String>)>,
        pub on_should_crawl_callback: Option<fn(_: &Page) -> bool>,
        pub crawl_id: Box<String>,
        /* private fields */
    }

Expand description

Represents a website to crawl and gather all links or page content.

    use spider::website::Website;
    let mut website = Website::new("http://example.com");
    website.crawl();
    // `Website` will be filled with links or pages when crawled. If you need pages with the resource
    // call the `website.scrape` method with `website.get_pages` instead.
    for link in website.get_links() {
        // do something
    }

Fields[§](#fields)
------------------

[§](#structfield.configuration)`configuration: [Box](https://doc.rust-lang.org/1.85.0/alloc/boxed/struct.Box.html "struct alloc::boxed::Box")<[Configuration](../configuration/struct.Configuration.html "struct spider::configuration::Configuration")>`

Configuration properties for website.

[§](#structfield.on_link_find_callback)`on_link_find_callback: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[fn](https://doc.rust-lang.org/1.85.0/std/primitive.fn.html)(_: [CaseInsensitiveString](../struct.CaseInsensitiveString.html "struct spider::CaseInsensitiveString"), _: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[String](https://doc.rust-lang.org/1.85.0/alloc/string/struct.String.html "struct alloc::string::String")>) -> ([CaseInsensitiveString](../struct.CaseInsensitiveString.html "struct spider::CaseInsensitiveString"), [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[String](https://doc.rust-lang.org/1.85.0/alloc/string/struct.String.html "struct alloc::string::String")>)>`

The callback when a link is found.

[§](#structfield.on_should_crawl_callback)`on_should_crawl_callback: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[fn](https://doc.rust-lang.org/1.85.0/std/primitive.fn.html)(_: &[Page](../page/struct.Page.html "struct spider::page::Page")) -> [bool](https://doc.rust-lang.org/1.85.0/std/primitive.bool.html)>`

The callback to use if a page should be ignored. Return false to ensure that the discovered links are not crawled.

[§](#structfield.crawl_id)`crawl_id: [Box](https://doc.rust-lang.org/1.85.0/alloc/boxed/struct.Box.html "struct alloc::boxed::Box")<[String](https://doc.rust-lang.org/1.85.0/alloc/string/struct.String.html "struct alloc::string::String")>`

Set the crawl ID to track. This allows explicit targeting for shutdown, pause, and etc.

Implementations[§](#implementations)
------------------------------------

[Source](../../src/spider/website.rs.html#303-6308)[§](#impl-Website)

### impl [Website](struct.Website.html "struct spider::website::Website")

[Source](../../src/spider/website.rs.html#305-329)

#### pub fn [new](#method.new)(url: &[str](https://doc.rust-lang.org/1.85.0/std/primitive.str.html)) -> Self

Initialize Website object with a start link to crawl.

[Source](../../src/spider/website.rs.html#332-348)

#### pub fn [set\_url](#method.set_url)(&mut self, url: &[str](https://doc.rust-lang.org/1.85.0/std/primitive.str.html)) -> &mut Self

Set the url of the website to re-use configuration and data.

[Source](../../src/spider/website.rs.html#351-354)

#### pub fn [set\_url\_only](#method.set_url_only)(&mut self, url: &[str](https://doc.rust-lang.org/1.85.0/std/primitive.str.html)) -> &mut Self

Set the direct url of the website to re-use configuration and data without parsing the domain.

[Source](../../src/spider/website.rs.html#357-359)

#### pub fn [target\_id](#method.target_id)(&self) -> [String](https://doc.rust-lang.org/1.85.0/alloc/string/struct.String.html "struct alloc::string::String")

Get the target id for a crawl. This takes the crawl ID and the url and concats it without delimiters.

[Source](../../src/spider/website.rs.html#374-379)

#### pub fn [setup\_disk](#method.setup_disk)(&mut self)

Setup SQLite. This does nothing with `disk` flag enabled.

[Source](../../src/spider/website.rs.html#581-593)

#### pub fn [is\_allowed](#method.is_allowed)(&mut self, link: &[CaseInsensitiveString](../struct.CaseInsensitiveString.html "struct spider::CaseInsensitiveString")) -> [ProcessLinkStatus](enum.ProcessLinkStatus.html "enum spider::website::ProcessLinkStatus")

return `true` if URL:

*   is not already crawled
*   is not over depth
*   is not over crawl budget
*   is optionally whitelisted
*   is not blacklisted
*   is not forbidden in robot.txt file (if parameter is defined)

[Source](../../src/spider/website.rs.html#628-644)

#### pub fn [is\_allowed\_budgetless](#method.is_allowed_budgetless)( &mut self, link: &[CaseInsensitiveString](../struct.CaseInsensitiveString.html "struct spider::CaseInsensitiveString"), ) -> [ProcessLinkStatus](enum.ProcessLinkStatus.html "enum spider::website::ProcessLinkStatus")

return `true` if URL:

*   is not already crawled
*   is not over depth
*   is optionally whitelisted
*   is not blacklisted
*   is not forbidden in robot.txt file (if parameter is defined)

[Source](../../src/spider/website.rs.html#700-712)

#### pub fn [is\_allowed\_default](#method.is_allowed_default)(&self, link: &[CompactString](../../compact_str/struct.CompactString.html "struct compact_str::CompactString")) -> [ProcessLinkStatus](enum.ProcessLinkStatus.html "enum spider::website::ProcessLinkStatus")

return `true` if URL:

*   is optionally whitelisted
*   is not blacklisted
*   is not forbidden in robot.txt file (if parameter is defined)

[Source](../../src/spider/website.rs.html#717-732)

#### pub fn [is\_allowed\_robots](#method.is_allowed_robots)(&self, link: &[str](https://doc.rust-lang.org/1.85.0/std/primitive.str.html)) -> [bool](https://doc.rust-lang.org/1.85.0/std/primitive.bool.html)

return `true` if URL:

*   is not forbidden in robot.txt file (if parameter is defined)

[Source](../../src/spider/website.rs.html#840-842)

#### pub fn [size](#method.size)(&self) -> [usize](https://doc.rust-lang.org/1.85.0/std/primitive.usize.html)

Amount of pages crawled in memory only. Use get\_size for full links between memory and disk.

[Source](../../src/spider/website.rs.html#852-868)

#### pub async fn [get\_size](#method.get_size)(&self) -> [usize](https://doc.rust-lang.org/1.85.0/std/primitive.usize.html)

Get the amount of resources collected.

[Source](../../src/spider/website.rs.html#871-873)

#### pub fn [drain\_extra\_links](#method.drain_extra_links)(&mut self) -> [Drain](../../hashbrown/set/struct.Drain.html "struct hashbrown::set::Drain")<'\_, [CaseInsensitiveString](../struct.CaseInsensitiveString.html "struct spider::CaseInsensitiveString")\>

Drain the extra links used for things like the sitemap.

[Source](../../src/spider/website.rs.html#876-878)

#### pub fn [get\_initial\_status\_code](#method.get_initial_status_code)(&self) -> &[StatusCode](../../http/status/struct.StatusCode.html "struct http::status::StatusCode")

Get the initial status code of the request

[Source](../../src/spider/website.rs.html#886-890)

#### pub fn [drain\_links](#method.drain_links)(&mut self) -> [Drain](../../hashbrown/set/struct.Drain.html "struct hashbrown::set::Drain")<'\_, [SymbolUsize](../../string_interner/symbol/struct.SymbolUsize.html "struct string_interner::symbol::SymbolUsize")\>

Drain the links visited.

[Source](../../src/spider/website.rs.html#908-910)

#### pub fn [drain\_signatures](#method.drain_signatures)(&mut self) -> [Drain](../../hashbrown/set/struct.Drain.html "struct hashbrown::set::Drain")<'\_, [u64](https://doc.rust-lang.org/1.85.0/std/primitive.u64.html)\>

Drain the signatures visited.

[Source](../../src/spider/website.rs.html#923-929)

#### pub fn [set\_extra\_links](#method.set_extra_links)( &mut self, extra\_links: [HashSet](../../hashbrown/set/struct.HashSet.html "struct hashbrown::set::HashSet")<[CaseInsensitiveString](../struct.CaseInsensitiveString.html "struct spider::CaseInsensitiveString")\>, ) -> &[HashSet](../../hashbrown/set/struct.HashSet.html "struct hashbrown::set::HashSet")<[CaseInsensitiveString](../struct.CaseInsensitiveString.html "struct spider::CaseInsensitiveString")\>

Set extra links to crawl. This could be used in conjuntion with ‘website.persist\_links’ to extend the crawl on the next run.

[Source](../../src/spider/website.rs.html#932-935)

#### pub async fn [clear\_all](#method.clear_all)(&mut self)

Clear all pages, disk, and links stored in memory.

[Source](../../src/spider/website.rs.html#938-943)

#### pub fn [clear](#method.clear)(&mut self)

Clear all pages and links stored.

[Source](../../src/spider/website.rs.html#946-948)

#### pub fn [get\_client](#method.get_client)(&self) -> &[Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[Client](../type.Client.html "type spider::Client")\>

Get the HTTP request client. The client is set after the crawl has started.

[Source](../../src/spider/website.rs.html#951-953)

#### pub fn [get\_pages](#method.get_pages)(&self) -> [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<&[Vec](https://doc.rust-lang.org/1.85.0/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")<[Page](../page/struct.Page.html "struct spider::page::Page")\>>

Page getter.

[Source](../../src/spider/website.rs.html#963-973)

#### pub async fn [get\_links\_disk](#method.get_links_disk)(&self) -> [HashSet](../../hashbrown/set/struct.HashSet.html "struct hashbrown::set::HashSet")<[CaseInsensitiveString](../struct.CaseInsensitiveString.html "struct spider::CaseInsensitiveString")\>

Links visited getter for disk. This does nothing with `disk` flag enabled.

[Source](../../src/spider/website.rs.html#977-984)

#### pub async fn [get\_all\_links\_visited](#method.get_all_links_visited)(&self) -> [HashSet](../../hashbrown/set/struct.HashSet.html "struct hashbrown::set::HashSet")<[CaseInsensitiveString](../struct.CaseInsensitiveString.html "struct spider::CaseInsensitiveString")\>

Links all the links visited between memory and disk.

[Source](../../src/spider/website.rs.html#993-995)

#### pub fn [get\_links](#method.get_links)(&self) -> [HashSet](../../hashbrown/set/struct.HashSet.html "struct hashbrown::set::HashSet")<[CaseInsensitiveString](../struct.CaseInsensitiveString.html "struct spider::CaseInsensitiveString")\>

Links visited getter for memory resources.

[Source](../../src/spider/website.rs.html#998-1000)

#### pub fn [get\_url\_parsed](#method.get_url_parsed)(&self) -> &[Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[Box](https://doc.rust-lang.org/1.85.0/alloc/boxed/struct.Box.html "struct alloc::boxed::Box")<[Url](../../url/struct.Url.html "struct url::Url")\>>

Domain parsed url getter.

[Source](../../src/spider/website.rs.html#1003-1005)

#### pub fn [get\_url](#method.get_url)(&self) -> &[CaseInsensitiveString](../struct.CaseInsensitiveString.html "struct spider::CaseInsensitiveString")

Domain name getter.

[Source](../../src/spider/website.rs.html#1008-1010)

#### pub fn [get\_delay](#method.get_delay)(&self) -> [Duration](https://doc.rust-lang.org/1.85.0/core/time/struct.Duration.html "struct core::time::Duration")

Crawl delay getter.

[Source](../../src/spider/website.rs.html#1013-1015)

#### pub fn [get\_status](#method.get_status)(&self) -> &[CrawlStatus](enum.CrawlStatus.html "enum spider::website::CrawlStatus")

Get the active crawl status.

[Source](../../src/spider/website.rs.html#1018-1021)

#### pub fn [reset\_status](#method.reset_status)(&mut self) -> &[CrawlStatus](enum.CrawlStatus.html "enum spider::website::CrawlStatus")

Reset the active crawl status to bypass websites that are blocked.

[Source](../../src/spider/website.rs.html#1025-1028)

#### pub fn [persist\_links](#method.persist_links)(&mut self) -> &mut Self

Set the crawl status to persist between the run. Example crawling a sitemap and all links after - website.crawl\_sitemap().await.persist\_links().crawl().await

[Source](../../src/spider/website.rs.html#1031-1049)

#### pub fn [get\_absolute\_path](#method.get_absolute_path)(&self, domain: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/1.85.0/std/primitive.str.html)\>) -> [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[Url](../../url/struct.Url.html "struct url::Url")\>

Absolute base url of crawl.

[Source](../../src/spider/website.rs.html#1052-1054)

#### pub fn [stop](#method.stop)(&mut self)

Stop all crawls for the website.

[Source](../../src/spider/website.rs.html#1062-1090)

#### pub async fn [configure\_robots\_parser](#method.configure_robots_parser)(&mut self, client: &[Client](../type.Client.html "type spider::Client"))

configure the robots parser on initial crawl attempt and run.

[Source](../../src/spider/website.rs.html#1406-1409)

#### pub fn [set\_http\_client](#method.set_http_client)(&mut self, client: [Client](../type.Client.html "type spider::Client")) -> &[Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[Client](../type.Client.html "type spider::Client")\>

Set the HTTP client to use directly. This is helpful if you manually call ‘website.configure\_http\_client’ before the crawl.

[Source](../../src/spider/website.rs.html#1413-1417)

#### pub fn [configure\_http\_client](#method.configure_http_client)(&self) -> [Client](../type.Client.html "type spider::Client")

Configure http client.

[Source](../../src/spider/website.rs.html#2801-2817)

#### pub async fn [crawl](#method.crawl)(&mut self)

Start to crawl website with async concurrency.

[Source](../../src/spider/website.rs.html#2820-2835)

#### pub async fn [crawl\_sitemap](#method.crawl_sitemap)(&mut self)

Start to crawl website with async concurrency using the sitemap. This does not page forward into the request. This does nothing without the `sitemap` flag enabled.

[Source](../../src/spider/website.rs.html#2861-2867)

#### pub async fn [configure\_setup](#method.configure_setup)(&mut self)

Configures the website crawling process for concurrent execution with the ability to send it across threads for subscriptions.

[Source](../../src/spider/website.rs.html#2871-2877)

#### pub fn [configure\_setup\_norobots](#method.configure_setup_norobots)(&mut self)

Configures the website crawling process for concurrent execution with the ability to send it across threads for subscriptions without robot protection. You can manually call `website.configure_robots_parser` after.

[Source](../../src/spider/website.rs.html#2884-2902)

#### pub async fn [crawl\_raw\_send](#method.crawl_raw_send)(&self, url: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/1.85.0/std/primitive.str.html)\>)

Initiates the website crawling http process concurrently with the ability to send it across threads for subscriptions. Ensure that `website.configure_setup()` has been called before executing this function. It checks the status to ensure it is not firewall-blocked before proceeding with concurrent crawling. You can pass in a manual url in order to setup a new crawl directly with pre-configurations ready.

[Source](../../src/spider/website.rs.html#3009-3011)

#### pub async fn [crawl\_smart](#method.crawl_smart)(&mut self)

Start to crawl website with async concurrency smart. Use HTTP first and JavaScript Rendering as needed. This has no effect without the `smart` flag enabled.

[Source](../../src/spider/website.rs.html#3014-3030)

#### pub async fn [crawl\_raw](#method.crawl_raw)(&mut self)

Start to crawl website with async concurrency using the base raw functionality. Useful when using the `chrome` feature and defaulting to the basic implementation.

[Source](../../src/spider/website.rs.html#3033-3061)

#### pub async fn [scrape](#method.scrape)(&mut self)

Start to scrape/download website with async concurrency.

[Source](../../src/spider/website.rs.html#3064-3091)

#### pub async fn [scrape\_raw](#method.scrape_raw)(&mut self)

Start to crawl website with async concurrency using the base raw functionality. Useful when using the “chrome” feature and defaulting to the basic implementation.

[Source](../../src/spider/website.rs.html#3094-3122)

#### pub async fn [scrape\_smart](#method.scrape_smart)(&mut self)

Start to scrape website with async concurrency smart. Use HTTP first and JavaScript Rendering as needed. This has no effect without the `smart` flag enabled.

[Source](../../src/spider/website.rs.html#3125-3153)

#### pub async fn [scrape\_sitemap](#method.scrape_sitemap)(&mut self)

Start to scrape website sitemap with async concurrency. Use HTTP first and JavaScript Rendering as needed. This has no effect without the `sitemap` flag enabled.

[Source](../../src/spider/website.rs.html#4819-4825)

#### pub async fn [sitemap\_crawl](#method.sitemap_crawl)( &mut self, \_client: &[Client](../type.Client.html "type spider::Client"), \_handle: &[Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[Arc](https://doc.rust-lang.org/1.85.0/alloc/sync/struct.Arc.html "struct alloc::sync::Arc")<[AtomicI8](https://doc.rust-lang.org/1.85.0/core/sync/atomic/struct.AtomicI8.html "struct core::sync::atomic::AtomicI8")\>>, \_scrape: [bool](https://doc.rust-lang.org/1.85.0/std/primitive.bool.html), )

Sitemap crawl entire lists. Note: this method does not re-crawl the links of the pages found on the sitemap. This does nothing without the `sitemap` flag.

[Source](../../src/spider/website.rs.html#5641-5645)

#### pub fn [with\_respect\_robots\_txt](#method.with_respect_robots_txt)(&mut self, respect\_robots\_txt: [bool](https://doc.rust-lang.org/1.85.0/std/primitive.bool.html)) -> &mut Self

Respect robots.txt file.

[Source](../../src/spider/website.rs.html#5648-5651)

#### pub fn [with\_subdomains](#method.with_subdomains)(&mut self, subdomains: [bool](https://doc.rust-lang.org/1.85.0/std/primitive.bool.html)) -> &mut Self

Include subdomains detection.

[Source](../../src/spider/website.rs.html#5654-5657)

#### pub fn [with\_tld](#method.with_tld)(&mut self, tld: [bool](https://doc.rust-lang.org/1.85.0/std/primitive.bool.html)) -> &mut Self

Include tld detection.

[Source](../../src/spider/website.rs.html#5660-5664)

#### pub fn [with\_http2\_prior\_knowledge](#method.with_http2_prior_knowledge)( &mut self, http2\_prior\_knowledge: [bool](https://doc.rust-lang.org/1.85.0/std/primitive.bool.html), ) -> &mut Self

Only use HTTP/2.

[Source](../../src/spider/website.rs.html#5667-5670)

#### pub fn [with\_delay](#method.with_delay)(&mut self, delay: [u64](https://doc.rust-lang.org/1.85.0/std/primitive.u64.html)) -> &mut Self

Delay between request as ms.

[Source](../../src/spider/website.rs.html#5673-5676)

#### pub fn [with\_request\_timeout](#method.with_request_timeout)( &mut self, request\_timeout: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[Duration](https://doc.rust-lang.org/1.85.0/core/time/struct.Duration.html "struct core::time::Duration")\>, ) -> &mut Self

Max time to wait for request.

[Source](../../src/spider/website.rs.html#5679-5683)

#### pub fn [with\_danger\_accept\_invalid\_certs](#method.with_danger_accept_invalid_certs)( &mut self, accept\_invalid\_certs: [bool](https://doc.rust-lang.org/1.85.0/std/primitive.bool.html), ) -> &mut Self

Dangerously accept invalid certificates - this should be used as a last resort.

[Source](../../src/spider/website.rs.html#5686-5689)

#### pub fn [with\_user\_agent](#method.with_user_agent)(&mut self, user\_agent: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/1.85.0/std/primitive.str.html)\>) -> &mut Self

Add user agent to request.

[Source](../../src/spider/website.rs.html#5692-5695)

#### pub fn [with\_preserve\_host\_header](#method.with_preserve_host_header)(&mut self, preserve: [bool](https://doc.rust-lang.org/1.85.0/std/primitive.bool.html)) -> &mut Self

Preserve the HOST header.

[Source](../../src/spider/website.rs.html#5706-5708)

#### pub fn [with\_sitemap](#method.with_sitemap)(&mut self, \_sitemap\_url: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/1.85.0/std/primitive.str.html)\>) -> &mut Self

Add user agent to request. This does nothing without the `sitemap` flag enabled.

[Source](../../src/spider/website.rs.html#5711-5714)

#### pub fn [with\_proxies](#method.with_proxies)(&mut self, proxies: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[Vec](https://doc.rust-lang.org/1.85.0/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")<[String](https://doc.rust-lang.org/1.85.0/alloc/string/struct.String.html "struct alloc::string::String")\>>) -> &mut Self

Use proxies for request.

[Source](../../src/spider/website.rs.html#5717-5723)

#### pub fn [with\_proxies\_direct](#method.with_proxies_direct)( &mut self, proxies: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[Vec](https://doc.rust-lang.org/1.85.0/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")<[RequestProxy](../configuration/struct.RequestProxy.html "struct spider::configuration::RequestProxy")\>>, ) -> &mut Self

Use proxies for request with control between chrome and http.

[Source](../../src/spider/website.rs.html#5726-5729)

#### pub fn [with\_concurrency\_limit](#method.with_concurrency_limit)(&mut self, limit: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/1.85.0/std/primitive.usize.html)\>) -> &mut Self

Set the concurrency limits. If you set the value to None to use the default limits using the system CPU cors \* n.

[Source](../../src/spider/website.rs.html#5733-5735)

#### pub fn [with\_crawl\_id](#method.with_crawl_id)(&mut self, \_crawl\_id: [String](https://doc.rust-lang.org/1.85.0/alloc/string/struct.String.html "struct alloc::string::String")) -> &mut Self

Set a crawl ID to use for tracking crawls. This does nothing without the `control` flag enabled.

[Source](../../src/spider/website.rs.html#5745-5751)

#### pub fn [with\_blacklist\_url](#method.with_blacklist_url)<T>( &mut self, blacklist\_url: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[Vec](https://doc.rust-lang.org/1.85.0/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")<T>>, ) -> &mut Self

where [Vec](https://doc.rust-lang.org/1.85.0/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")<[CompactString](../../compact_str/struct.CompactString.html "struct compact_str::CompactString")\>: [From](https://doc.rust-lang.org/1.85.0/core/convert/trait.From.html "trait core::convert::From")<[Vec](https://doc.rust-lang.org/1.85.0/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")<T>>,

Add blacklist urls to ignore.

[Source](../../src/spider/website.rs.html#5754-5757)

#### pub fn [with\_retry](#method.with_retry)(&mut self, retry: [u8](https://doc.rust-lang.org/1.85.0/std/primitive.u8.html)) -> &mut Self

Set the retry limit for request. Set the value to 0 for no retries. The default is 0.

[Source](../../src/spider/website.rs.html#5760-5763)

#### pub fn [with\_no\_control\_thread](#method.with_no_control_thread)(&mut self, no\_control\_thread: [bool](https://doc.rust-lang.org/1.85.0/std/primitive.bool.html)) -> &mut Self

Skip setting up a control thread for pause, start, and shutdown programmatic handling. This does nothing without the \[control\] flag enabled.

[Source](../../src/spider/website.rs.html#5766-5772)

#### pub fn [with\_whitelist\_url](#method.with_whitelist_url)<T>( &mut self, blacklist\_url: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[Vec](https://doc.rust-lang.org/1.85.0/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")<T>>, ) -> &mut Self

where [Vec](https://doc.rust-lang.org/1.85.0/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")<[CompactString](../../compact_str/struct.CompactString.html "struct compact_str::CompactString")\>: [From](https://doc.rust-lang.org/1.85.0/core/convert/trait.From.html "trait core::convert::From")<[Vec](https://doc.rust-lang.org/1.85.0/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")<T>>,

Add whitelist urls to allow.

[Source](../../src/spider/website.rs.html#5775-5778)

#### pub fn [with\_headers](#method.with_headers)(&mut self, headers: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[HeaderMap](../../http/header/map/struct.HeaderMap.html "struct http::header::map::HeaderMap")\>) -> &mut Self

Set HTTP headers for request using [reqwest::header::HeaderMap](https://docs.rs/reqwest/latest/reqwest/header/struct.HeaderMap.html).

[Source](../../src/spider/website.rs.html#5781-5784)

#### pub fn [with\_budget](#method.with_budget)(&mut self, budget: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[HashMap](../../hashbrown/map/struct.HashMap.html "struct hashbrown::map::HashMap")<&[str](https://doc.rust-lang.org/1.85.0/std/primitive.str.html), [u32](https://doc.rust-lang.org/1.85.0/std/primitive.u32.html)\>>) -> &mut Self

Set a crawl budget per path with levels support /a/b/c or for all paths with “\*”. This does nothing without the `budget` flag enabled.

[Source](../../src/spider/website.rs.html#5787-5789)

#### pub fn [set\_crawl\_budget](#method.set_crawl_budget)( &mut self, budget: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[HashMap](../../hashbrown/map/struct.HashMap.html "struct hashbrown::map::HashMap")<[CaseInsensitiveString](../struct.CaseInsensitiveString.html "struct spider::CaseInsensitiveString"), [u32](https://doc.rust-lang.org/1.85.0/std/primitive.u32.html)\>>, )

Set the crawl budget directly. This does nothing without the `budget` flag enabled.

[Source](../../src/spider/website.rs.html#5792-5795)

#### pub fn [with\_depth](#method.with_depth)(&mut self, depth: [usize](https://doc.rust-lang.org/1.85.0/std/primitive.usize.html)) -> &mut Self

Set a crawl depth limit. If the value is 0 there is no limit.

[Source](../../src/spider/website.rs.html#5798-5804)

#### pub fn [with\_external\_domains](#method.with_external_domains)<'a, 'b>( &mut self, external\_domains: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<impl [Iterator](https://doc.rust-lang.org/1.85.0/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [String](https://doc.rust-lang.org/1.85.0/alloc/string/struct.String.html "struct alloc::string::String")\> + 'a>, ) -> &mut Self

Group external domains to treat the crawl as one. If None is passed this will clear all prior domains.

[Source](../../src/spider/website.rs.html#5807-5818)

#### pub fn [with\_on\_link\_find\_callback](#method.with_on_link_find_callback)( &mut self, on\_link\_find\_callback: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[fn](https://doc.rust-lang.org/1.85.0/std/primitive.fn.html)(\_: [CaseInsensitiveString](../struct.CaseInsensitiveString.html "struct spider::CaseInsensitiveString"), \_: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[String](https://doc.rust-lang.org/1.85.0/alloc/string/struct.String.html "struct alloc::string::String")\>) -> ([CaseInsensitiveString](../struct.CaseInsensitiveString.html "struct spider::CaseInsensitiveString"), [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[String](https://doc.rust-lang.org/1.85.0/alloc/string/struct.String.html "struct alloc::string::String")\>)>, ) -> &mut Self

Perform a callback to run on each link find.

[Source](../../src/spider/website.rs.html#5821-5830)

#### pub fn [with\_on\_should\_crawl\_callback](#method.with_on_should_crawl_callback)( &mut self, on\_should\_crawl\_callback: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[fn](https://doc.rust-lang.org/1.85.0/std/primitive.fn.html)(\_: &[Page](../page/struct.Page.html "struct spider::page::Page")) -> [bool](https://doc.rust-lang.org/1.85.0/std/primitive.bool.html)\>, ) -> &mut Self

Use a callback to determine if a page should be ignored. Return false to ensure that the discovered links are not crawled.

[Source](../../src/spider/website.rs.html#5833-5836)

#### pub fn [with\_cookies](#method.with_cookies)(&mut self, cookie\_str: &[str](https://doc.rust-lang.org/1.85.0/std/primitive.str.html)) -> &mut Self

Cookie string to use in request. This does nothing without the `cookies` flag enabled.

[Source](../../src/spider/website.rs.html#5839-5842)

#### pub fn [with\_cron](#method.with_cron)(&mut self, cron\_str: &[str](https://doc.rust-lang.org/1.85.0/std/primitive.str.html), cron\_type: [CronType](enum.CronType.html "enum spider::website::CronType")) -> &mut Self

Setup cron jobs to run. This does nothing without the `cron` flag enabled.

[Source](../../src/spider/website.rs.html#5845-5848)

#### pub fn [with\_locale](#method.with_locale)(&mut self, locale: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[String](https://doc.rust-lang.org/1.85.0/alloc/string/struct.String.html "struct alloc::string::String")\>) -> &mut Self

Overrides default host system locale with the specified one. This does nothing without the `chrome` flag enabled.

[Source](../../src/spider/website.rs.html#5851-5854)

#### pub fn [with\_stealth](#method.with_stealth)(&mut self, stealth\_mode: [bool](https://doc.rust-lang.org/1.85.0/std/primitive.bool.html)) -> &mut Self

Use stealth mode for the request. This does nothing without the `chrome` flag enabled.

[Source](../../src/spider/website.rs.html#5857-5860)

#### pub fn [with\_openai](#method.with_openai)(&mut self, openai\_configs: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[GPTConfigs](../features/openai_common/struct.GPTConfigs.html "struct spider::features::openai_common::GPTConfigs")\>) -> &mut Self

Use OpenAI to get dynamic javascript to drive the browser. This does nothing without the `openai` flag enabled.

[Source](../../src/spider/website.rs.html#5863-5866)

#### pub fn [with\_caching](#method.with_caching)(&mut self, cache: [bool](https://doc.rust-lang.org/1.85.0/std/primitive.bool.html)) -> &mut Self

Cache the page following HTTP rules. This method does nothing if the `cache` feature is not enabled.

[Source](../../src/spider/website.rs.html#5869-5872)

#### pub fn [with\_service\_worker\_enabled](#method.with_service_worker_enabled)(&mut self, enabled: [bool](https://doc.rust-lang.org/1.85.0/std/primitive.bool.html)) -> &mut Self

Enable or disable Service Workers. This method does nothing if the `chrome` feature is not enabled.

[Source](../../src/spider/website.rs.html#5875-5878)

#### pub fn [with\_fingerprint](#method.with_fingerprint)(&mut self, fingerprint: [bool](https://doc.rust-lang.org/1.85.0/std/primitive.bool.html)) -> &mut Self

Setup custom fingerprinting for chrome. This method does nothing if the `chrome` feature is not enabled.

[Source](../../src/spider/website.rs.html#5881-5884)

#### pub fn [with\_viewport](#method.with_viewport)(&mut self, viewport: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[Viewport](../features/chrome_common/struct.Viewport.html "struct spider::features::chrome_common::Viewport")\>) -> &mut Self

Configures the viewport of the browser, which defaults to 800x600. This method does nothing if the `chrome` feature is not enabled.

[Source](../../src/spider/website.rs.html#5887-5894)

#### pub fn [with\_wait\_for\_idle\_network](#method.with_wait_for_idle_network)( &mut self, wait\_for\_idle\_network: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[WaitForIdleNetwork](../features/chrome_common/struct.WaitForIdleNetwork.html "struct spider::features::chrome_common::WaitForIdleNetwork")\>, ) -> &mut Self

Wait for idle network request. This method does nothing if the `chrome` feature is not enabled.

[Source](../../src/spider/website.rs.html#5897-5903)

#### pub fn [with\_wait\_for\_selector](#method.with_wait_for_selector)( &mut self, wait\_for\_selector: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[WaitForSelector](../features/chrome_common/struct.WaitForSelector.html "struct spider::features::chrome_common::WaitForSelector")\>, ) -> &mut Self

Wait for a CSS query selector. This method does nothing if the `chrome` feature is not enabled.

[Source](../../src/spider/website.rs.html#5906-5912)

#### pub fn [with\_wait\_for\_idle\_dom](#method.with_wait_for_idle_dom)( &mut self, wait\_for\_selector: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[WaitForSelector](../features/chrome_common/struct.WaitForSelector.html "struct spider::features::chrome_common::WaitForSelector")\>, ) -> &mut Self

Wait for idle dom mutations for target element. This method does nothing if the `chrome` feature is not enabled.

[Source](../../src/spider/website.rs.html#5915-5921)

#### pub fn [with\_wait\_for\_delay](#method.with_wait_for_delay)( &mut self, wait\_for\_delay: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[WaitForDelay](../features/chrome_common/struct.WaitForDelay.html "struct spider::features::chrome_common::WaitForDelay")\>, ) -> &mut Self

Wait for a delay. Should only be used for testing. This method does nothing if the `chrome` feature is not enabled.

[Source](../../src/spider/website.rs.html#5924-5927)

#### pub fn [with\_redirect\_limit](#method.with_redirect_limit)(&mut self, redirect\_limit: [usize](https://doc.rust-lang.org/1.85.0/std/primitive.usize.html)) -> &mut Self

Set the max redirects allowed for request.

[Source](../../src/spider/website.rs.html#5930-5933)

#### pub fn [with\_redirect\_policy](#method.with_redirect_policy)(&mut self, policy: [RedirectPolicy](../configuration/enum.RedirectPolicy.html "enum spider::configuration::RedirectPolicy")) -> &mut Self

Set the redirect policy to use, either Strict or Loose by default.

[Source](../../src/spider/website.rs.html#5936-5943)

#### pub fn [with\_chrome\_intercept](#method.with_chrome_intercept)( &mut self, chrome\_intercept: [RequestInterceptConfiguration](../features/chrome_common/struct.RequestInterceptConfiguration.html "struct spider::features::chrome_common::RequestInterceptConfiguration"), ) -> &mut Self

Use request intercept for the request to only allow content that matches the host. If the content is from a 3rd party it needs to be part of our include list. This method does nothing if the `chrome_intercept` flag is not enabled.

[Source](../../src/spider/website.rs.html#5946-5949)

#### pub fn [with\_full\_resources](#method.with_full_resources)(&mut self, full\_resources: [bool](https://doc.rust-lang.org/1.85.0/std/primitive.bool.html)) -> &mut Self

Determine whether to collect all the resources found on pages.

[Source](../../src/spider/website.rs.html#5952-5955)

#### pub fn [with\_ignore\_sitemap](#method.with_ignore_sitemap)(&mut self, ignore\_sitemap: [bool](https://doc.rust-lang.org/1.85.0/std/primitive.bool.html)) -> &mut Self

Ignore the sitemap when crawling. This method does nothing if the `sitemap` flag is not enabled.

[Source](../../src/spider/website.rs.html#5958-5961)

#### pub fn [with\_timezone\_id](#method.with_timezone_id)(&mut self, timezone\_id: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[String](https://doc.rust-lang.org/1.85.0/alloc/string/struct.String.html "struct alloc::string::String")\>) -> &mut Self

Overrides default host system timezone with the specified one. This does nothing without the `chrome` flag enabled.

[Source](../../src/spider/website.rs.html#5964-5972)

#### pub fn [with\_evaluate\_on\_new\_document](#method.with_evaluate_on_new_document)( &mut self, evaluate\_on\_new\_document: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[Box](https://doc.rust-lang.org/1.85.0/alloc/boxed/struct.Box.html "struct alloc::boxed::Box")<[String](https://doc.rust-lang.org/1.85.0/alloc/string/struct.String.html "struct alloc::string::String")\>>, ) -> &mut Self

Set a custom script to evaluate on new document creation. This does nothing without the feat flag `chrome` enabled.

[Source](../../src/spider/website.rs.html#5975-5978)

#### pub fn [with\_limit](#method.with_limit)(&mut self, limit: [u32](https://doc.rust-lang.org/1.85.0/std/primitive.u32.html)) -> &mut Self

Set a crawl page limit. If the value is 0 there is no limit.

[Source](../../src/spider/website.rs.html#5981-5987)

#### pub fn [with\_screenshot](#method.with_screenshot)( &mut self, screenshot\_config: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[ScreenShotConfig](../features/chrome_common/struct.ScreenShotConfig.html "struct spider::features::chrome_common::ScreenShotConfig")\>, ) -> &mut Self

Set the chrome screenshot configuration. This does nothing without the `chrome` flag enabled.

[Source](../../src/spider/website.rs.html#5990-5993)

#### pub fn [with\_shared\_queue](#method.with_shared_queue)(&mut self, shared\_queue: [bool](https://doc.rust-lang.org/1.85.0/std/primitive.bool.html)) -> &mut Self

Use a shared semaphore to evenly handle workloads. The default is false.

[Source](../../src/spider/website.rs.html#5996-6003)

#### pub fn [with\_auth\_challenge\_response](#method.with_auth_challenge_response)( &mut self, auth\_challenge\_response: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[AuthChallengeResponse](../features/chrome_common/struct.AuthChallengeResponse.html "struct spider::features::chrome_common::AuthChallengeResponse")\>, ) -> &mut Self

Set the authentiation challenge response. This does nothing without the feat flag `chrome` enabled.

[Source](../../src/spider/website.rs.html#6006-6009)

#### pub fn [with\_return\_page\_links](#method.with_return_page_links)(&mut self, return\_page\_links: [bool](https://doc.rust-lang.org/1.85.0/std/primitive.bool.html)) -> &mut Self

Return the links found on the page in the channel subscriptions. This method does nothing if the `decentralized` is enabled.

[Source](../../src/spider/website.rs.html#6012-6016)

#### pub fn [with\_chrome\_connection](#method.with_chrome_connection)( &mut self, chrome\_connection\_url: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[String](https://doc.rust-lang.org/1.85.0/alloc/string/struct.String.html "struct alloc::string::String")\>, ) -> &mut Self

Set the connection url for the chrome instance. This method does nothing if the `chrome` is not enabled.

[Source](../../src/spider/website.rs.html#6019-6025)

#### pub fn [with\_execution\_scripts](#method.with_execution_scripts)( &mut self, execution\_scripts: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[ExecutionScriptsMap](../features/chrome_common/type.ExecutionScriptsMap.html "type spider::features::chrome_common::ExecutionScriptsMap")\>, ) -> &mut Self

Set JS to run on certain pages. This method does nothing if the `chrome` is not enabled.

[Source](../../src/spider/website.rs.html#6028-6035)

#### pub fn [with\_automation\_scripts](#method.with_automation_scripts)( &mut self, automation\_scripts: [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[AutomationScriptsMap](../features/chrome_common/type.AutomationScriptsMap.html "type spider::features::chrome_common::AutomationScriptsMap")\>, ) -> &mut Self

Run web automated actions on certain pages. This method does nothing if the `chrome` is not enabled.

[Source](../../src/spider/website.rs.html#6038-6041)

#### pub fn [with\_block\_assets](#method.with_block_assets)(&mut self, only\_html: [bool](https://doc.rust-lang.org/1.85.0/std/primitive.bool.html)) -> &mut Self

Block assets from loading from the network. Focus primarly on HTML documents.

[Source](../../src/spider/website.rs.html#6044-6047)

#### pub fn [with\_normalize](#method.with_normalize)(&mut self, normalize: [bool](https://doc.rust-lang.org/1.85.0/std/primitive.bool.html)) -> &mut Self

Normalize the content de-duplicating trailing slash pages and other pages that can be duplicated. This may initially show the link in your links\_visited or subscription calls but, the following links will not be crawled.

[Source](../../src/spider/website.rs.html#6050-6053)

#### pub fn [with\_config](#method.with_config)(&mut self, config: [Configuration](../configuration/struct.Configuration.html "struct spider::configuration::Configuration")) -> &mut Self

Set the configuration for the website directly.

[Source](../../src/spider/website.rs.html#6056-6062)

#### pub fn [build](#method.build)(&self) -> [Result](https://doc.rust-lang.org/1.85.0/core/result/enum.Result.html "enum core::result::Result")<Self, Self>

Build the website configuration when using with\_builder.

[Source](../../src/spider/website.rs.html#6152-6168)

#### pub fn [subscribe](#method.subscribe)(&mut self, capacity: [usize](https://doc.rust-lang.org/1.85.0/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[Receiver](../../tokio/sync/broadcast/struct.Receiver.html "struct tokio::sync::broadcast::Receiver")<[Page](../page/struct.Page.html "struct spider::page::Page")\>>

Sets up a subscription to receive concurrent data. This will panic if it is larger than `usize::MAX / 2`. Set the value to `0` to use the semaphore permits. If the subscription is going to block or use async methods, make sure to spawn a task to avoid losing messages. This does nothing unless the `sync` flag is enabled.

##### [§](#examples)Examples

Subscribe and receive messages using an async tokio environment:

    use spider::{tokio, website::Website};
    
    #[tokio::main]
    async fn main() {
        let mut website = Website::new("http://example.com");
        let mut rx = website.subscribe(0).unwrap();
    
        tokio::spawn(async move {
            while let Ok(page) = rx.recv().await {
                tokio::spawn(async move {
                    // Process the received page.
                    // If performing non-blocking tasks or managing a high subscription count, configure accordingly.
                });
            }
        });
    
        website.crawl().await;
    }

[Source](../../src/spider/website.rs.html#6172-6179)

#### pub fn [queue](#method.queue)(&mut self, capacity: [usize](https://doc.rust-lang.org/1.85.0/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[Sender](../../tokio/sync/broadcast/struct.Sender.html "struct tokio::sync::broadcast::Sender")<[String](https://doc.rust-lang.org/1.85.0/alloc/string/struct.String.html "struct alloc::string::String")\>>

Get a sender for queueing extra links mid crawl. This does nothing unless the `sync` flag is enabled.

[Source](../../src/spider/website.rs.html#6196-6198)

#### pub fn [unsubscribe](#method.unsubscribe)(&mut self)

Remove subscriptions for data. This is useful for auto droping subscriptions that are running on another thread. This does nothing without the `sync` flag enabled.

[Source](../../src/spider/website.rs.html#6278-6282)

#### pub fn [subscribe\_guard](#method.subscribe_guard)(&mut self) -> [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<[ChannelGuard](struct.ChannelGuard.html "struct spider::website::ChannelGuard")\>

Setup subscription counter to track concurrent operation completions. This helps keep a chrome instance active until all operations are completed from all threads to safely take screenshots and other actions. Make sure to call `inc` if you take a guard. Without calling `inc` in the subscription receiver the crawl will stay in a infinite loop. This does nothing without the `sync` flag enabled. You also need to use the ‘chrome\_store\_page’ to keep the page alive between request.

##### [§](#example)Example

    use spider::tokio;
    use spider::website::Website;
    
    #[tokio::main]
    async fn main() {
        let mut website: Website = Website::new("http://example.com");
        let mut rx2 = website.subscribe(18).unwrap();
        let mut rxg = website.subscribe_guard().unwrap();
    
        tokio::spawn(async move {
            while let Ok(page) = rx2.recv().await {
                println!("📸 - {:?}", page.get_url());
                page
                    .screenshot(
                        true,
                        true,
                        spider::configuration::CaptureScreenshotFormat::Png,
                        Some(75),
                        None::<std::path::PathBuf>,
                        None,
                    )
                    .await;
                rxg.inc();
            }
        });
        website.crawl().await;
    }

[Source](../../src/spider/website.rs.html#6295-6297)

#### pub fn [get\_crawl\_id](#method.get_crawl_id)(&self) -> [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<&[Box](https://doc.rust-lang.org/1.85.0/alloc/boxed/struct.Box.html "struct alloc::boxed::Box")<[String](https://doc.rust-lang.org/1.85.0/alloc/string/struct.String.html "struct alloc::string::String")\>>

Get the attached crawl id.

Trait Implementations[§](#trait-implementations)
------------------------------------------------

[Source](../../src/spider/website.rs.html#256)[§](#impl-Clone-for-Website)

### impl [Clone](https://doc.rust-lang.org/1.85.0/core/clone/trait.Clone.html "trait core::clone::Clone") for [Website](struct.Website.html "struct spider::website::Website")

[Source](../../src/spider/website.rs.html#256)[§](#method.clone)

#### fn [clone](https://doc.rust-lang.org/1.85.0/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [Website](struct.Website.html "struct spider::website::Website")

Returns a copy of the value. [Read more](https://doc.rust-lang.org/1.85.0/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · [Source](https://doc.rust-lang.org/1.85.0/src/core/clone.rs.html#174)[§](#method.clone_from)

#### fn [clone\_from](https://doc.rust-lang.org/1.85.0/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/1.85.0/core/clone/trait.Clone.html#method.clone_from)

[Source](../../src/spider/website.rs.html#256)[§](#impl-Debug-for-Website)

### impl [Debug](https://doc.rust-lang.org/1.85.0/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Website](struct.Website.html "struct spider::website::Website")

[Source](../../src/spider/website.rs.html#256)[§](#method.fmt)

#### fn [fmt](https://doc.rust-lang.org/1.85.0/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/1.85.0/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/1.85.0/core/fmt/type.Result.html "type core::fmt::Result")

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/1.85.0/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](../../src/spider/website.rs.html#256)[§](#impl-Default-for-Website)

### impl [Default](https://doc.rust-lang.org/1.85.0/core/default/trait.Default.html "trait core::default::Default") for [Website](struct.Website.html "struct spider::website::Website")

[Source](../../src/spider/website.rs.html#256)[§](#method.default)

#### fn [default](https://doc.rust-lang.org/1.85.0/core/default/trait.Default.html#tymethod.default)() -> [Website](struct.Website.html "struct spider::website::Website")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/1.85.0/core/default/trait.Default.html#tymethod.default)

[Source](../../src/spider/website.rs.html#6413-6423)[§](#impl-Display-for-Website)

### impl [Display](https://doc.rust-lang.org/1.85.0/core/fmt/trait.Display.html "trait core::fmt::Display") for [Website](struct.Website.html "struct spider::website::Website")

[Source](../../src/spider/website.rs.html#6414-6422)[§](#method.fmt-1)

#### fn [fmt](https://doc.rust-lang.org/1.85.0/core/fmt/trait.Display.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/1.85.0/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/1.85.0/core/fmt/type.Result.html "type core::fmt::Result")

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/1.85.0/core/fmt/trait.Display.html#tymethod.fmt)

[Source](../../src/spider/website.rs.html#6425)[§](#impl-Error-for-Website)

### impl [Error](https://doc.rust-lang.org/1.85.0/core/error/trait.Error.html "trait core::error::Error") for [Website](struct.Website.html "struct spider::website::Website")

1.30.0 · [Source](https://doc.rust-lang.org/1.85.0/src/core/error.rs.html#81)[§](#method.source)

#### fn [source](https://doc.rust-lang.org/1.85.0/core/error/trait.Error.html#method.source)(&self) -> [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Error](https://doc.rust-lang.org/1.85.0/core/error/trait.Error.html "trait core::error::Error") + 'static)>

Returns the lower-level source of this error, if any. [Read more](https://doc.rust-lang.org/1.85.0/core/error/trait.Error.html#method.source)

1.0.0 · [Source](https://doc.rust-lang.org/1.85.0/src/core/error.rs.html#107)[§](#method.description)

#### fn [description](https://doc.rust-lang.org/1.85.0/core/error/trait.Error.html#method.description)(&self) -> &[str](https://doc.rust-lang.org/1.85.0/std/primitive.str.html)

👎Deprecated since 1.42.0: use the Display impl or to\_string()

[Read more](https://doc.rust-lang.org/1.85.0/core/error/trait.Error.html#method.description)

1.0.0 · [Source](https://doc.rust-lang.org/1.85.0/src/core/error.rs.html#117)[§](#method.cause)

#### fn [cause](https://doc.rust-lang.org/1.85.0/core/error/trait.Error.html#method.cause)(&self) -> [Option](https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html "enum core::option::Option")<&dyn [Error](https://doc.rust-lang.org/1.85.0/core/error/trait.Error.html "trait core::error::Error")\>

👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting

[Source](https://doc.rust-lang.org/1.85.0/src/core/error.rs.html#180)[§](#method.provide)

#### fn [provide](https://doc.rust-lang.org/1.85.0/core/error/trait.Error.html#method.provide)<'a>(&'a self, request: &mut [Request](https://doc.rust-lang.org/1.85.0/core/error/struct.Request.html "struct core::error::Request")<'a>)

🔬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. [Read more](https://doc.rust-lang.org/1.85.0/core/error/trait.Error.html#method.provide)

Auto Trait Implementations[§](#synthetic-implementations)
---------------------------------------------------------

[§](#impl-Freeze-for-Website)

### impl ![Freeze](https://doc.rust-lang.org/1.85.0/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Website](struct.Website.html "struct spider::website::Website")

[§](#impl-RefUnwindSafe-for-Website)

### impl ![RefUnwindSafe](https://doc.rust-lang.org/1.85.0/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Website](struct.Website.html "struct spider::website::Website")

[§](#impl-Send-for-Website)

### impl [Send](https://doc.rust-lang.org/1.85.0/core/marker/trait.Send.html "trait core::marker::Send") for [Website](struct.Website.html "struct spider::website::Website")

[§](#impl-Sync-for-Website)

### impl [Sync](https://doc.rust-lang.org/1.85.0/core/marker/trait.Sync.html "trait core::marker::Sync") for [Website](struct.Website.html "struct spider::website::Website")

[§](#impl-Unpin-for-Website)

### impl [Unpin](https://doc.rust-lang.org/1.85.0/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Website](struct.Website.html "struct spider::website::Website")

[§](#impl-UnwindSafe-for-Website)

### impl ![UnwindSafe](https://doc.rust-lang.org/1.85.0/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Website](struct.Website.html "struct spider::website::Website")

Blanket Implementations[§](#blanket-implementations)
----------------------------------------------------

[Source](https://doc.rust-lang.org/1.85.0/src/core/any.rs.html#138)[§](#impl-Any-for-T)

### impl<T> [Any](https://doc.rust-lang.org/1.85.0/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/1.85.0/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/1.85.0/src/core/any.rs.html#139)[§](#method.type_id)

#### fn [type\_id](https://doc.rust-lang.org/1.85.0/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/1.85.0/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/1.85.0/core/any/trait.Any.html#tymethod.type_id)

[Source](https://doc.rust-lang.org/1.85.0/src/core/borrow.rs.html#209)[§](#impl-Borrow%3CT%3E-for-T)

### impl<T> [Borrow](https://doc.rust-lang.org/1.85.0/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/1.85.0/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/1.85.0/src/core/borrow.rs.html#211)[§](#method.borrow)

#### fn [borrow](https://doc.rust-lang.org/1.85.0/core/borrow/trait.Borrow.html#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/1.85.0/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/1.85.0/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/1.85.0/src/core/borrow.rs.html#217)[§](#impl-BorrowMut%3CT%3E-for-T)

### impl<T> [BorrowMut](https://doc.rust-lang.org/1.85.0/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/1.85.0/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/1.85.0/src/core/borrow.rs.html#218)[§](#method.borrow_mut)

#### fn [borrow\_mut](https://doc.rust-lang.org/1.85.0/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/1.85.0/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/1.85.0/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://doc.rust-lang.org/1.85.0/src/core/clone.rs.html#273)[§](#impl-CloneToUninit-for-T)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/1.85.0/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T

where T: [Clone](https://doc.rust-lang.org/1.85.0/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/1.85.0/src/core/clone.rs.html#275)[§](#method.clone_to_uninit)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/1.85.0/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)(&self, dst: [\*mut](https://doc.rust-lang.org/1.85.0/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/1.85.0/std/primitive.u8.html))

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dst`. [Read more](https://doc.rust-lang.org/1.85.0/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://doc.rust-lang.org/1.85.0/src/core/convert/mod.rs.html#767)[§](#impl-From%3CT%3E-for-T)

### impl<T> [From](https://doc.rust-lang.org/1.85.0/core/convert/trait.From.html "trait core::convert::From")<T> for T

[Source](https://doc.rust-lang.org/1.85.0/src/core/convert/mod.rs.html#770)[§](#method.from)

#### fn [from](https://doc.rust-lang.org/1.85.0/core/convert/trait.From.html#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](../../src/tracing/instrument.rs.html#325)[§](#impl-Instrument-for-T)

### impl<T> [Instrument](../../tracing/instrument/trait.Instrument.html "trait tracing::instrument::Instrument") for T

[Source](../../src/tracing/instrument.rs.html#86)[§](#method.instrument)

#### fn [instrument](../../tracing/instrument/trait.Instrument.html#method.instrument)(self, span: [Span](../../tracing/span/struct.Span.html "struct tracing::span::Span")) -> [Instrumented](../../tracing/instrument/struct.Instrumented.html "struct tracing::instrument::Instrumented")<Self>

Instruments this type with the provided [`Span`](../../tracing/span/struct.Span.html "struct tracing::span::Span"), returning an `Instrumented` wrapper. [Read more](../../tracing/instrument/trait.Instrument.html#method.instrument)

[Source](../../src/tracing/instrument.rs.html#128)[§](#method.in_current_span)

#### fn [in\_current\_span](../../tracing/instrument/trait.Instrument.html#method.in_current_span)(self) -> [Instrumented](../../tracing/instrument/struct.Instrumented.html "struct tracing::instrument::Instrumented")<Self>

Instruments this type with the [current](../../tracing/span/struct.Span.html#method.current "associated function tracing::span::Span::current") [`Span`](../../tracing/span/struct.Span.html "struct tracing::span::Span"), returning an `Instrumented` wrapper. [Read more](../../tracing/instrument/trait.Instrument.html#method.in_current_span)

[Source](https://doc.rust-lang.org/1.85.0/src/core/convert/mod.rs.html#750-752)[§](#impl-Into%3CU%3E-for-T)

### impl<T, U> [Into](https://doc.rust-lang.org/1.85.0/core/convert/trait.Into.html "trait core::convert::Into")<U> for T

where U: [From](https://doc.rust-lang.org/1.85.0/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://doc.rust-lang.org/1.85.0/src/core/convert/mod.rs.html#760)[§](#method.into)

#### fn [into](https://doc.rust-lang.org/1.85.0/core/convert/trait.Into.html#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From](https://doc.rust-lang.org/1.85.0/core/convert/trait.From.html "trait core::convert::From")<T> for U` chooses to do.

[Source](../../src/either/into_either.rs.html#64)[§](#impl-IntoEither-for-T)

### impl<T> [IntoEither](../../either/into_either/trait.IntoEither.html "trait either::into_either::IntoEither") for T

[Source](../../src/either/into_either.rs.html#29)[§](#method.into_either)

#### fn [into\_either](../../either/into_either/trait.IntoEither.html#method.into_either)(self, into\_left: [bool](https://doc.rust-lang.org/1.85.0/std/primitive.bool.html)) -> [Either](../../either/enum.Either.html "enum either::Either")<Self, Self>

Converts `self` into a [`Left`](../../either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](../../either/enum.Either.html "enum either::Either") if `into_left` is `true`. Converts `self` into a [`Right`](../../either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](../../either/enum.Either.html "enum either::Either") otherwise. [Read more](../../either/into_either/trait.IntoEither.html#method.into_either)

[Source](../../src/either/into_either.rs.html#55-57)[§](#method.into_either_with)

#### fn [into\_either\_with](../../either/into_either/trait.IntoEither.html#method.into_either_with)<F>(self, into\_left: F) -> [Either](../../either/enum.Either.html "enum either::Either")<Self, Self>

where F: [FnOnce](https://doc.rust-lang.org/1.85.0/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self) -> [bool](https://doc.rust-lang.org/1.85.0/std/primitive.bool.html),

Converts `self` into a [`Left`](../../either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](../../either/enum.Either.html "enum either::Either") if `into_left(&self)` returns `true`. Converts `self` into a [`Right`](../../either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](../../either/enum.Either.html "enum either::Either") otherwise. [Read more](../../either/into_either/trait.IntoEither.html#method.into_either_with)

[Source](../../src/typenum/type_operators.rs.html#34)[§](#impl-Same-for-T)

### impl<T> [Same](../../typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](../../src/typenum/type_operators.rs.html#35)[§](#associatedtype.Output)

#### type [Output](../../typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](../../src/compact_str/traits.rs.html#89)[§](#impl-ToCompactString-for-T)

### impl<T> [ToCompactString](../../compact_str/traits/trait.ToCompactString.html "trait compact_str::traits::ToCompactString") for T

where T: [Display](https://doc.rust-lang.org/1.85.0/core/fmt/trait.Display.html "trait core::fmt::Display"),

[Source](../../src/compact_str/traits.rs.html#91)[§](#method.try_to_compact_string)

#### fn [try\_to\_compact\_string](../../compact_str/traits/trait.ToCompactString.html#tymethod.try_to_compact_string)(&self) -> [Result](https://doc.rust-lang.org/1.85.0/core/result/enum.Result.html "enum core::result::Result")<[CompactString](../../compact_str/struct.CompactString.html "struct compact_str::CompactString"), [ToCompactStringError](../../compact_str/enum.ToCompactStringError.html "enum compact_str::ToCompactStringError")\>

Fallible version of [`ToCompactString::to_compact_string()`](../../compact_str/traits/trait.ToCompactString.html#method.to_compact_string "method compact_str::traits::ToCompactString::to_compact_string") [Read more](../../compact_str/traits/trait.ToCompactString.html#tymethod.try_to_compact_string)

[Source](../../src/compact_str/traits.rs.html#52)[§](#method.to_compact_string)

#### fn [to\_compact\_string](../../compact_str/traits/trait.ToCompactString.html#method.to_compact_string)(&self) -> [CompactString](../../compact_str/struct.CompactString.html "struct compact_str::CompactString")

Converts the given value to a [`CompactString`](../../compact_str/struct.CompactString.html "struct compact_str::CompactString"). [Read more](../../compact_str/traits/trait.ToCompactString.html#method.to_compact_string)

[Source](https://doc.rust-lang.org/1.85.0/src/alloc/borrow.rs.html#82-84)[§](#impl-ToOwned-for-T)

### impl<T> [ToOwned](https://doc.rust-lang.org/1.85.0/alloc/borrow/trait.ToOwned.html "trait alloc::borrow::ToOwned") for T

where T: [Clone](https://doc.rust-lang.org/1.85.0/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/1.85.0/src/alloc/borrow.rs.html#86)[§](#associatedtype.Owned)

#### type [Owned](https://doc.rust-lang.org/1.85.0/alloc/borrow/trait.ToOwned.html#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/1.85.0/src/alloc/borrow.rs.html#87)[§](#method.to_owned)

#### fn [to\_owned](https://doc.rust-lang.org/1.85.0/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](https://doc.rust-lang.org/1.85.0/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/1.85.0/src/alloc/borrow.rs.html#91)[§](#method.clone_into)

#### fn [clone\_into](https://doc.rust-lang.org/1.85.0/alloc/borrow/trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/1.85.0/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](https://doc.rust-lang.org/1.85.0/alloc/borrow/trait.ToOwned.html#method.clone_into)

[Source](https://doc.rust-lang.org/1.85.0/src/alloc/string.rs.html#2677)[§](#impl-ToString-for-T)

### impl<T> [ToString](https://doc.rust-lang.org/1.85.0/alloc/string/trait.ToString.html "trait alloc::string::ToString") for T

where T: [Display](https://doc.rust-lang.org/1.85.0/core/fmt/trait.Display.html "trait core::fmt::Display") + ?[Sized](https://doc.rust-lang.org/1.85.0/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/1.85.0/src/alloc/string.rs.html#2679)[§](#method.to_string)

#### fn [to\_string](https://doc.rust-lang.org/1.85.0/alloc/string/trait.ToString.html#tymethod.to_string)(&self) -> [String](https://doc.rust-lang.org/1.85.0/alloc/string/struct.String.html "struct alloc::string::String")

Converts the given value to a `String`. [Read more](https://doc.rust-lang.org/1.85.0/alloc/string/trait.ToString.html#tymethod.to_string)

[Source](https://doc.rust-lang.org/1.85.0/src/core/convert/mod.rs.html#807-809)[§](#impl-TryFrom%3CU%3E-for-T)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/1.85.0/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U> for T

where U: [Into](https://doc.rust-lang.org/1.85.0/core/convert/trait.Into.html "trait core::convert::Into")<T>,

[Source](https://doc.rust-lang.org/1.85.0/src/core/convert/mod.rs.html#811)[§](#associatedtype.Error-1)

#### type [Error](https://doc.rust-lang.org/1.85.0/core/convert/trait.TryFrom.html#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/1.85.0/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/1.85.0/src/core/convert/mod.rs.html#814)[§](#method.try_from)

#### fn [try\_from](https://doc.rust-lang.org/1.85.0/core/convert/trait.TryFrom.html#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/1.85.0/core/result/enum.Result.html "enum core::result::Result")<T, <T as [TryFrom](https://doc.rust-lang.org/1.85.0/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U>>::[Error](https://doc.rust-lang.org/1.85.0/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://doc.rust-lang.org/1.85.0/src/core/convert/mod.rs.html#792-794)[§](#impl-TryInto%3CU%3E-for-T)

### impl<T, U> [TryInto](https://doc.rust-lang.org/1.85.0/core/convert/trait.TryInto.html "trait core::convert::TryInto")<U> for T

where U: [TryFrom](https://doc.rust-lang.org/1.85.0/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>,

[Source](https://doc.rust-lang.org/1.85.0/src/core/convert/mod.rs.html#796)[§](#associatedtype.Error)

#### type [Error](https://doc.rust-lang.org/1.85.0/core/convert/trait.TryInto.html#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/1.85.0/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/1.85.0/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/1.85.0/src/core/convert/mod.rs.html#799)[§](#method.try_into)

#### fn [try\_into](https://doc.rust-lang.org/1.85.0/core/convert/trait.TryInto.html#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/1.85.0/core/result/enum.Result.html "enum core::result::Result")<U, <U as [TryFrom](https://doc.rust-lang.org/1.85.0/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/1.85.0/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](../../src/tracing/instrument.rs.html#393)[§](#impl-WithSubscriber-for-T)

### impl<T> [WithSubscriber](../../tracing/instrument/trait.WithSubscriber.html "trait tracing::instrument::WithSubscriber") for T

[Source](../../src/tracing/instrument.rs.html#176-178)[§](#method.with_subscriber)

#### fn [with\_subscriber](../../tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../../tracing/instrument/struct.WithDispatch.html "struct tracing::instrument::WithDispatch")<Self>

where S: [Into](https://doc.rust-lang.org/1.85.0/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../../tracing_core/dispatcher/struct.Dispatch.html "struct tracing_core::dispatcher::Dispatch")\>,

Attaches the provided [`Subscriber`](../../tracing_core/subscriber/trait.Subscriber.html "trait tracing_core::subscriber::Subscriber") to this type, returning a [`WithDispatch`](../../tracing/instrument/struct.WithDispatch.html "struct tracing::instrument::WithDispatch") wrapper. [Read more](../../tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](../../src/tracing/instrument.rs.html#228)[§](#method.with_current_subscriber)

#### fn [with\_current\_subscriber](../../tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../../tracing/instrument/struct.WithDispatch.html "struct tracing::instrument::WithDispatch")<Self>

Attaches the current [default](../../tracing/dispatcher/index.html#setting-the-default-subscriber "mod tracing::dispatcher") [`Subscriber`](../../tracing_core/subscriber/trait.Subscriber.html "trait tracing_core::subscriber::Subscriber") to this type, returning a [`WithDispatch`](../../tracing/instrument/struct.WithDispatch.html "struct tracing::instrument::WithDispatch") wrapper. [Read more](../../tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[Source](../../src/yoke/erased.rs.html#22)[§](#impl-ErasedDestructor-for-T)

### impl<T> [ErasedDestructor](../../yoke/erased/trait.ErasedDestructor.html "trait yoke::erased::ErasedDestructor") for T

where T: 'static,

[Source](../../src/icu_provider/any.rs.html#32)[§](#impl-MaybeSendSync-for-T)

### impl<T> [MaybeSendSync](../../icu_provider/any/trait.MaybeSendSync.html "trait icu_provider::any::MaybeSendSync") for T