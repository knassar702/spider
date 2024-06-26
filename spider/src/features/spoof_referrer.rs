#[cfg(feature = "spoof")]
lazy_static! {
    /// A list of websites that are common
    // we may want to move this to a new repo like ua_generator.
    static ref WEBSITES: [&'static str; 200] = [
        "https://www.google.com",
        "https://www.facebook.com",
        "https://www.amazon.com",
        "https://www.reddit.com",
        "https://www.youtube.com",
        "https://www.wikipedia.org",
        "https://www.twitter.com",
        "https://www.instagram.com",
        "https://www.linkedin.com",
        "https://www.netflix.com",
        "https://www.spotify.com",
        "https://www.apple.com",
        "https://www.microsoft.com",
        "https://www.yahoo.com",
        "https://www.imgur.com",
        "https://www.adobe.com",
        "https://www.tumblr.com",
        "https://www.pinterest.com",
        "https://www.ebay.com",
        "https://www.craigslist.org",
        "https://www.bing.com",
        "https://www.office.com",
        "https://www.qq.com",
        "https://www.taobao.com",
        "https://www.sohu.com",
        "https://www.vk.com",
        "https://www.gitlab.com",
        "https://www.wordpress.org",
        "https://www.github.com",
        "https://www.aliexpress.com",
        "https://www.whatsapp.com",
        "https://www.weibo.com",
        "https://www.etsy.com",
        "https://www.shutterstock.com",
        "https://www.dropbox.com",
        "https://www.quora.com",
        "https://www.cloudflare.com",
        "https://www.soundcloud.com",
        "https://www.paypal.com",
        "https://www.medium.com",
        "https://www.alibaba.com",
        "https://www.huffpost.com",
        "https://www.expedia.com",
        "https://www.tripadvisor.com",
        "https://www.cnn.com",
        "https://www.foxnews.com",
        "https://www.bbc.com",
        "https://www.nytimes.com",
        "https://www.theguardian.com",
        "https://www.walmart.com",
        "https://www.target.com",
        "https://www.sears.com",
        "https://www.bestbuy.com",
        "https://www.macy's.com",
        "https://www.lowes.com",
        "https://www.homdepot.com",
        "https://www.jcpenny.com",
        "https://www.kohls.com",
        "https://www.starbucks.com",
        "https://www.zappos.com",
        "https://www.ikea.com",
        "https://www.nike.com",
        "https://www.adidas.com",
        "https://www.underarmour.com",
        "https://www.puma.com",
        "https://www.sony.com",
        "https://www.samsung.com",
        "https://www.panasonic.com",
        "https://www.lg.com",
        "https://www.pepsico.com",
        "https://www.cocacola.com",
        "https://www.mcdonalds.com",
        "https://www.burgerking.com",
        "https://www.pizzahut.com",
        "https://www.dominos.com",
        "https://www.kfc.com",
        "https://www.subway.com",
        "https://www.reuters.com",
        "https://www.time.com",
        "https://www.forbes.com",
        "https://www.businessinsider.com",
        "https://www.bloomberg.com",
        "https://www.wsj.com",
        "https://www.usatoday.com",
        "https://www.newsweek.com",
        "https://www.nbcnews.com",
        "https://www.dailymail.co.uk",
        "https://www.thetimes.co.uk",
        "https://www.nationalgeographic.com",
        "https://www.npr.org",
        "https://www.techcrunch.com",
        "https://www.engadget.com",
        "https://www.wired.com",
        "https://www.gizmodo.com",
        "https://www.theverge.com",
        "https://www.slashdot.org",
        "https://www.fiverr.com",
        "https://www.upwork.com",
        "https://www.toptal.com",
        "https://www.glassdoor.com",
        "https://www.indeed.com",
        "https://www.monster.com",
        "https://www.simplyhired.com",
        "https://www.zillow.com",
        "https://www.realtor.com",
        "https://www.trulia.com",
        "https://www.redfin.com",
        "https://www.apartments.com",
        "https://www.rent.com",
        "https://www.cars.com",
        "https://www.autotrader.com",
        "https://www.kbb.com",
        "https://www.carvana.com",
        "https://www.truecar.com",
        "https://www.edmunds.com",
        "https://www.orbitz.com",
        "https://www.priceline.com",
        "https://www.hotels.com",
        "https://www.booking.com",
        "https://www.travelocity.com",
        "https://www.kayak.com",
        "https://www.jetblue.com",
        "https://www.southwest.com",
        "https://www.united.com",
        "https://www.delta.com",
        "https://www.americanairlines.com",
        "https://www.spirit.com",
        "https://www.gamestop.com",
        "https://www.ign.com",
        "https://www.gamespot.com",
        "https://www.twitch.tv",
        "https://www.steampowered.com",
        "https://www.epicgames.com",
        "https://www.ea.com",
        "https://www.blizzard.com",
        "https://www.rockstargames.com",
        "https://www.nintendo.com",
        "https://www.playstation.com",
        "https://www.xbox.com",
        "https://www.sega.com",
        "https://www.bethesda.net",
        "https://www.riotgames.com",
        "https://www.ubisoft.com",
        "https://www.activision.com",
        "https://www.capcom.com",
        "https://www.square-enix.com",
        "https://www.bioware.com",
        "https://www.zynga.com",
        "https://www.supercell.com",
        "https://www.king.com",
        "https://www.moonton.com",
        "https://www.zenithbank.com",
        "https://www.cbsnews.com",
        "https://www.weather.com",
        "https://www.accuweather.com",
        "https://www.nationalweather.org",
        "https://www.healthline.com",
        "https://www.mayoclinic.org",
        "https://www.webmd.com",
        "https://www.nih.gov",
        "https://www.cdc.gov",
        "https://www.who.int",
        "https://www.medicalnewstoday.com",
        "https://www.sciencedaily.com",
        "https://www.sciencemag.org",
        "https://www.nature.com",
        "https://www.arxiv.org",
        "https://www.jstor.org",
        "https://www.academia.edu",
        "https://www.researchgate.net",
        "https://www.springer.com",
        "https://www.elsevier.com",
        "https://www.wiley.com",
        "https://www.tandfonline.com",
        "https://www.sciencedirect.com",
        "https://www.moodle.org",
        "https://www.khanacademy.org",
        "https://www.edx.org",
        "https://www.coursera.org",
        "https://www.udemy.com",
        "https://www.skillshare.com",
        "https://www.lynda.com",
        "https://www.linuxfoundation.org",
        "https://www.gnu.org",
        "https://www.apache.org",
        "https://www.opensource.org",
        "https://www.mozilla.org",
        "https://www.howstuffworks.com",
        "https://www.ehow.com",
        "https://www.diy.org",
        "https://www.thisoldhouse.com",
        "https://www.gutenberg.org",
        "https://www.archive.org",
        "https://www.smithsonianmag.com",
        "https://www.duolingo.com",
        "https://www.rosettastone.com",
        "https://www.babbel.com",
        "https://www.memrise.com",
        "https://www.busuu.com",
        "https://www.livemocha.com",
    ];
}

#[cfg(feature = "spoof")]
/// Get a random website from a static precompiled list.
pub fn spoof_referrer() -> &'static str {
    WEBSITES[fastrand::usize(..WEBSITES.len())]
}

#[cfg(not(feature = "spoof"))]
/// Get a random website from a static precompiled list.
pub fn spoof_referrer() -> &'static str {
    ""
}
