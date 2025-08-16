:root {
  --darkblue: rgb(10, 49, 68);

  --blue: rgb(3, 83, 99);

  --lightblue: rgb(12, 105, 121);

  --purple: rgb(142, 96, 117);

  --pink: rgb(191, 85, 105);

  --red: rgb(218, 59, 59);

  --orange: rgb(207, 121, 100);

  --yellow: rgb(247, 240, 194);

  --white: rgb(255, 255, 255);

  --paper_white: rgb(251, 248, 228);

  --diff-green: hsl(122, 61%, 87%);

  --diff-red: hsl(0, 86%, 27%);

  --diff-red: hsl(0, 86%, 27%);

  --green: hsl(79, 100%, 50%);
}

:root {
  --base: 0.8rem;
  --width: 960px;
  --arrow-height: calc(2 * var(--base));
  --body-font-width: 300;
  --body-font-weight: 300;
  --body-font-slant: 0;
  --bold-font-weight: 800;
  --heading-lineheight: 1;
  --heading-size: 500;
  --heading-quad: 0;
  --heading-bevl: 0;
  --heading-oval: 1000;
  --title-size: 0;
  --title-quad: 0;
  --title-bevl: 0;
  --title-oval: 0;
  --subheading-size: 0;
  --subheading-quad: 0;
  --subheading-bevl: 0;
  --subheading-oval: 0;
  --grid-gutter: var(--base);
  --grid-height: calc(2 * var(--base));
  --avatar-size: calc(12 * var(--base));
  --social-icon: var(--grid-height);
  --noise-size: 64;
  {{ $image := resources.Get "images/arrow.svg" }}
  --arrow-file: url({{ $image.RelPermalink }});
  --arrow-right-file: url(/assets/arrow-right.fd936d11.svg);
}

:root {
  --maincolor: orange;
  --bordercl: rebeccapurple;
  --callouctcolor: dodgerblue;
  --hovercolor: navy;
  --darkMaincolor: #50fa7b;
}

@font-face {
  font-family: C64_Pro-STYLE;
  {{ $font := resources.Get "fonts/C64_Pro-STYLE.woff2" }}
  src: url({{ $font.RelPermalink }});
}

html {
  color: #232333;
  font-family: "Roboto Mono", monospace;
  font-size: 15px;
  line-height: 1.6em;
}
body {
  display: block;
  margin: 8px;
}
* {
  -webkit-tap-highlight-color: rgba(0, 0, 0, 0);
}

::selection {
  background: var(--maincolor);
  color: #fff;
}

/* Containers */
.content {
  margin-top: 4em;
  margin-bottom: 4em;
  margin-left: auto;
  margin-right: auto;
  max-width: 800px;
  padding: 0 1ch;
  word-wrap: break-word;
}

/* Header */
header,
footer {
  --spacing: var(--grid-height);
  background: var(--white);
  color: var(--orange);
  padding-top: 1px;
  display: flex;
  flex-wrap: wrap;
  justify-content: stretch;
  align-items: flex-end;
  padding: var(--spacing);
  position: relative;
}

header > h1 {
  margin: 0;
  margin-top: 0; /*calc(1 * var(--grid-height));*/
  padding: 0;
  flex-basis: 50%;
  transform-origin: 0% 100%;
  transform: rotate(2deg);
  font-variation-settings:
    "size" var(--title-size),
    "quad" var(--title-quad),
    "bevl" var(--title-bevl),
    "oval" var(--title-oval);
}

header > h1 > a {
  text-decoration: none;
  font-family: C64_Pro-STYLE;
  font-size: 1.8em;
}

header > h1 > a:link {
  color: var(--orange);
}
header > h1 > a:active {
  color: var(--orange);
}
header > h1 > a:visited {
  color: var(--orange);
}
header > h1 > a:hover {
  color: var(--yellow);
}

header {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
  margin: 1em 0;
  line-height: 1.5em;
}

header .main {
  font-size: 1.5rem;
}
h1,
h2,
h3,
h4,
h5,
h6 {
  font-size: 1.2rem;
  margin-top: 2em;
}

.content h1::before {
  color: var(--maincolor);
  content: "# ";
}
h2::before {
  color: var(--maincolor);
  content: "## ";
}
h3::before {
  color: var(--maincolor);
  content: "### ";
}
h4::before {
  color: var(--maincolor);
  content: "#### ";
}
h5::before {
  color: var(--maincolor);
  content: "##### ";
}
h6::before {
  color: var(--maincolor);
  content: "###### ";
}

header > .arrows,
footer > .arrows {
  flex-basis: calc(100% + 2 * var(--spacing));
  margin: calc(-1 * var(--spacing));
}
header > .arrows {
  margin-top: calc(1 * var(--grid-height));
}

.arrow {
  position: relative;
  height: var(--grid-height);
  background: var(--color);
}
.arrow:after {
  content: "";
  display: block;
  position: absolute;
  left: 0;
  width: 100%;
  height: var(--arrow-height);
  background: var(--color);
  mask: var(--mask);
  -webkit-mask: var(--mask);
  top: 100%;
  --mask: var(--arrow-file) 0% 100%/100% 200%;
}
.arrow:nth-of-type(1) {
  z-index: 9;
}
.arrow:nth-of-type(2) {
  z-index: 8;
}
.arrow:nth-of-type(3) {
  z-index: 7;
}
.arrow:nth-of-type(4) {
  z-index: 6;
}
.arrow:nth-of-type(5) {
  z-index: 5;
}
.arrow:nth-of-type(6) {
  z-index: 4;
}
.arrow:nth-of-type(7) {
  z-index: 3;
}
.arrow.darkblue {
  --color: var(--darkblue);
}
.arrow.blue {
  --color: var(--blue);
}
.arrow.lightblue {
  --color: var(--lightblue);
}
.arrow.purple {
  --color: var(--purple);
}
.arrow.pink {
  --color: var(--pink);
}
.arrow.red {
  --color: hsl(15.9, 64.8%, 49%);
}
.arrow.orange_light {
  --color: hsl(37.8, 78.9%, 68.4%);
}
.arrow.orange {
  --color: hsl(30.2, 73.4%, 52.7%);
}
.arrow.green {
  --color: hsl(51.3, 26.6%, 40.6%);
}
.arrow.yellow {
  --color: var(--yellow);
}
.arrow.white {
  --color: var(--white);
}

footer {
  width: 100%;
}
.footer {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  padding: 10px;
  margin-top: 10em;
}

pre > code,
.codestyle > code {
  display: inline-block;
  margin: 0 auto;
  box-sizing: border-box;
  width: 100%;
  overflow-x: auto;
  font-family:
    Consolas,
    Monaco,
    Andale Mono,
    Ubuntu Mono,
    monospace;
}

pre,
.codestyle,
figure {
  margin: calc(var(--grid-height) + 2 * var(--grid-gutter)) 0;
}

:not(pre) > code {
  color: var(--blue);
  white-space: nowrap;
}
pre,
.codestyle {
  --color: var(--white);
  position: relative;
  display: block;
  margin-left: calc((100vw - var(--width)) / -2);
  margin-right: calc((100vw - var(--width)) / -2);
  background-color: var(--color);
  padding: var(--grid-height) 0;
  tab-size: 4;
}
pre:before,
pre:after,
.codestyle:before,
.codestyle:after {
  content: "";
  display: block;
  position: absolute;
  left: 0;
  width: 100%;
  height: var(--arrow-height);
  background: inherit;
  mask: var(--mask);
  -webkit-mask: var(--mask);
}
pre:before,
.codestyle:before {
  top: calc(var(--arrow-height) * -1);
  --mask: var(--arrow-file) 0% 0%/100% 200%;
}
pre:after,
.codestyle:after {
  top: 100%;
  --mask: var(--arrow-file) 0% 100%/100% 200%;
}
pre + figcaption {
  position: relative;
  display: block;
  --color: var(--white);
  background-color: var(--color);
  margin-left: calc((100vw - var(--width)) / -2);
  margin-right: calc((100vw - var(--width)) / -2);
  margin-top: calc(-2 * var(--grid-height));
  margin-bottom: calc(2 * var(--grid-height));
  max-width: initial;
  padding: var(--grid-height) calc(50vw - var(--width) / 2);
}

pre + figcaption:before,
pre + figcaption:after {
  content: "";
  display: block;
  position: absolute;
  left: 0;
  width: 100%;
  height: var(--arrow-height);
  background: inherit;
  mask: var(--mask);
  -webkit-mask: var(--mask);
}

pre + figcaption:before {
  top: calc(var(--arrow-height) * -1);
  --mask: var(--arrow-file) 0% 0%/100% 200%;
}

pre + figcaption:after {
  top: 100%;
  --mask: var(--arrow-file) 0% 100%/100% 200%;
}

pre > code,
.codestyle > code {
  color: #93a1a1;
  padding: var(--grid-gutter);
  padding-left: calc(50vw - var(--width) / 2);
  padding-right: calc(50vw - var(--width) / 2);
}

