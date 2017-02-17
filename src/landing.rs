/* man-browsed - Server for viewing HTML man pages.
 * Copyright ©2017 Dmitri Kourennyi
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.

 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.

 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

pub static HTML: &'static str = r#"<!DOCTYPE html>
<html>
<head>
 <title>Local Man Page Viewer</title>
 <link rel="search" type="application/opensearchdescription+xml"
       title="Search Local Man Pages" href="os.xml">
</head>
<body>
 <form method="get">
  <input name="q"></input>
  <input type="submit"></input>
 </form>
 <h2>Browser Integration</h2>
 <h3>Firefox</h3>
 <ul>
  <li>Click on the search icon in the search bar.</li>
  <li>Click on 'Add "Search Local Man Pages"' to add the search provider.</li>
  <li>Click on 'Change Search Settings' button.</li>
  <li>Double-click the 'Keyword' column for the 'Man Browse' search provider.</li>
  <li>Type in 'man' or an alternate keyword of your choosing.</li>
  <li>In the awesomebar, just type in 'man TERM' to instantly get a manual page.</li>
 </ul>
</body>
</html>
"#;

pub static OSEARCH: &'static str = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSearchDescription xmlns="http://a9.com/-/spec/opensearch/1.1/">
   <ShortName>Man Browse</ShortName>
   <Description>View local man pages in your browser.</Description>
   <Tags>man manual</Tags>
   <Url type="text/html" template="http://$addr:$port/?q={searchTerms}"/>
 </OpenSearchDescription>
"#;
