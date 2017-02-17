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
