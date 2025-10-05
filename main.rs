use std::io;

fn christian() {
	let mut input = String::new(); //defines input
	println!("What aspect about Christianity do you wish to learn about?\n");
	println!("a) Basics | b) Doctrine | c) Christ | d) Denominations");

	io::stdin()
		.read_line(&mut input)
		.expect("Failed to read line"); //in-case of any errors

	let specifics_christian = input.trim(); //so we can call the input afterwards
	if specifics_christian == "a" {
		println!("Christianity is a religion formed in the 1st century AD, and was mainly founded by YWH, also known as Jehova, Jesus, Christ, etcetera. The basics of Christ are that he was born to the Virgin Mary; he grew up a Jew (and in a historical context, was likely a carpenter) however, during his lifetime, he consistently preached a different kind of Judiasm. The twelve aposles, then, believed that he was the coming of the Son of God (though, continued to maintain that they were monotheistic, as is judaism) and, eventually, Jesus was betrayed by one of his aposles, and died on the cross, which has now become the symbol for christianity.");
	}
	if specifics_christian == "b" {
		println!("Christians (generally) believe in a couple of things, and while these vary by denomination (see 'd' for details) it generally goes something as follows, 1. Jesus died for our sins, meaning that we can sin (mostly interpreted as 'doing a bad thing' however can also be interpreted as 'going against god, or seeking independence from god') and that we can be forgiven. 2. Jesus is the son of god and will return someday (however, some denominations disagree on how and why it will happen, as the book of revelations is quite complex and artisic) 3. The trinity* God, the Son (Jesus), and the Holy Sprit (some denominations believe in only God and Jesus, while some may only believe in Jesus, some in Jesus and the Holy Spirit, some call it the Holy Ghost instead of the Holy Spirit, but generally it's accepted that those are the main ideas.) 4. Jesus taught the people about the afterlife. (i.e. Heaven and Hell*)");
	}
	if specifics_christian == "c" {
		println!("Christ is the central figurehead of Christian religion. All denominations accept that Christ is the Son (or sometimes, he is) of God. This is similar to the way that the Jewish religion looks at Moses, or the Islamic religion looks at Muhammad, with some slight variations. Generally, Christians focus on the death of Jesus--as Jesus knew that he was going to die, and yet still came to earth in order to make sure that every one of us has the ability to be forgiven.");
	}
	if specifics_christian == "d" {
		println!("There are many different denominations of christians, so many in-fact that I cannot list them all properly, with all respects to their own ideas, and so, unforunetly, I will have to list them in groups rather than individual denominations.\n");
		println!("Orthodox Christians: Catholic, Old Catholic, Sedevacantism (a niche catholic subsect) and Eastern Orthodox\n");
		println!("Protestant:\n New Religions (>1830): Adventist, Jehovah's Witness* (is disputed as a cult) Latter Day Saints/THe Church of Jesus Christ of Latter-day Saints* (also disputed as a cult) Baptist, Methodist\n Earlier Protestant: Anglican, Lutheran, Moravian/Hussite Pentecostal, Quaker* (is mostly dead, but played an influential rule in early america) and Calvinists");
		println!("Others/Restorationist Churches (The belief that the original word of christ has been altered and has been restored by said religion): Christadelphians, Iglesia ni Cristo, Irvingian Church, Jehovah's Witness* (disputed as a cult), Latter Day Saints* (disputed as a cult) The New Church, and Unitarians/Universalists.");
	} else {
		println!("You seem to have typed in an errenious answer (likely spelled wrong, it should be all lower-case!)");
	}
}

fn islam() {
	let mut input = String::new(); //defines input
	println!("What spects about Islam do you wish to learn about?\n");
	println!("a) Basics | b) Doctrine | c) Muhammad | d) Denominations");

	io::stdin()
		.read_line(&mut input)
		.expect("Failed to read line"); //in-case of any errors

	let specifics_islam = input.trim();
	if specifics_islam == "a" {
		println!("Islam was founded in the 7th Century by Muhammad. Islam is an Abrahamic Religion, meaning that it is related to Christianity and Judaism, Islam being the most recent of these religions. The basic idea of Islam is that sometime during the reformations during ~3->5 Centuries, the word of God was distorted and altered, and that Muhammad had been commanded to revive the gospel, and that it was necessary to keep this preserved. For this reason, Classical Arabic has been preserved since the 7th Century, and the Qu'ran (the holy text written by Muhammad) is the direct word from him, not translated into different languages or altered. However, it's important to remember that the religion of Islam is *strictly* Monotheistic, with a very strong focus on Allah (the arabic word for 'God') being One, this is in slight opposition to Christians with the idea of the Trinity, and more akin to Judaism, which is also Monotheistic.");
	}
	if specifics_islam == "b" {
		println!("(The teachings of Islam are generally more difficult to gain an understanding of unless you speak/can read in Classical Arabic or MSA, so forgive me if some of this information is distorted or not exactly correct) The teachings of Islam can be broken down, generally into 3 different ideas, the first and most prevenalant is the idea of Monotheism, Islam places a very, very large focus on Allah being one; in fact, most depicitons of Muhammad refrain from showing his face in order to prevent people from worshiping Muhammad, rather than Allah. The second is the idea that the Qu'ran is the word of God, and untainted. This is also a large focus of Islam. The third is that we need to actively worship Allah in order to gain eternal gratitude and enlightenment (though generally not called enlightenment, usually described as salvation, or some other save-based word).");
	}
	if specifics_islam == "c" {
		println!("Muhammad was the founded of Islam, who lived in the 7th Century CE. He claimed that he had been revealed two important things, 1. That the original teachings of Abrahamic Religions (Christianity and Judaism) had been altered to the point of inaccuracy, and 2. That he waws tasked with rewritten the Torah/Bible in order to perfectly fit the word of God. After he had completed the Qu'ran, he became a Sidhartha-like teacher, who taught that the Qu'ran was, in-fact, the word of God. This was the foundation of the Second Largest Religion in the World, and likley Largest in due-time. (As of October 2025, forgive me if this project is not updated when that event occurs.)");
	}
	if specifics_islam == "d" {
		println!("There are not as many denominations of Islam compared to something like Christianity. Here are them listed.\n");
		println!("Sunni (~85% of Islam) -- Hanafi, Hanbali, Maliki, and Shafi'i\n");
		println!("Shi'ah (~10% of Islam) -- Isma'lism, Jafri, and Zaidiyyah\n");
		println!("Ibadiyya (~2.5% of Islam) -- Nukkar and Azzabas\n");
		println!("(It's also important to remember about non-denominational Muslims, while not a common occurance, still something that happens.)");
	} else {
		println!("You have typed in an erronious answer, your answer should be a simple, one letter response, lower-case. I.e. 'd', or 'a'. Please Try Again.");
	}
}
fn hinduism() {
	println!("This is incomplete, please try again later. I am sorry for the inconvience.");
}
fn buddhism() {
	println!("This is incomplete, please try again later. I am sorry for the inconvience.");
}
fn jainism() {
	println!("This is incomplete, please try again later. I am sorry for the inconvience.");
}
fn bahai() {
	println!("This is incomplete, please try again later. I am sorry for the inconvience.");
}
fn judaism() {
	println!("This is incomplete, please try again later. I am sorry for the inconvience.");
}

fn main() {
	let mut input = String::new(); // Defines input for later use
	println!("Please input the religion you want to learn about:");

	io::stdin() // Actually uses the input
	.read_line(&mut input)
	.expect("Failed to read line"); // In case of any errors

	let religion_selection = input.trim(); // So we can call the input afterwards

	match religion_selection { // Easier to add more than an if statement
		"Christianity" => christian(),
		"Islam" => islam(),
		"Hinduism" => hinduism(),
		"Buddhism" => buddhism(),
		"Jainism" => jainism(),
		"Baha'i" => bahai(),
		"Judaism" => judaism(),
		_ => println!("Unknown religion selected"), // Handling unmatched input
	}
}
