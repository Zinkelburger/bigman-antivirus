# Go:
- ClamTK GUI no longer maintained
https://github.com/Cisco-Talos/clamav
https://github.com/dave-theunsub/clamtk

https://github.com/ossec/ossec-hids
https://cisofy.com/lynis/
Rewrite UI in Fyne?

# Browser Extension:

- Detect mismatch between <a> tag and its contents
Some gTLDs can have digits, fix the regex, etc.
Won't match paths (/page.html)

- Detect lack of dictionary matches
micrasoft.com
microsft.com

- ADGuard has a list of suspicious urls it uses?

- Detect mixing uppercase and lowercase letters, lowercase everything
neccdi.org vs neccdl.org
appie.com

# PDF:
- Suspicious segments
JS, check if URI matches link
Phishing via pdfs is very common. E.g. game tester gets a pdf for a game, but its the wrong website to download from
`pdfid`
`pdf-parser.py` to extract the malicious parts
- detect obfuscated code. Is it really that hard
dictonary words to chars ratio
not that simple, because you can shorten JS for valid reasons

# Crypto:
- Check to make sure addresses are correct
- White list of addresses, prevent modification, idk
How do people send crypto, how are the addresses being modified? Can we detect these modifications
Can we trick crypto extensions, lol. Like "put a crypto address up, does it get modified?" and detect tem

# Bash:
- All the time people have you install stuff like `wget website.com | bash`
instead, can we do `wget website.com | bigman`
tells you what will be executed
