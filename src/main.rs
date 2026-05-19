use error_chain ::error_chain;
use select :: document ::Document;
use select :: predicate ::Name;


error_chain {
    foreign_links {
   ReqError(request::Error);
   IoError(std ::io::Error);
    }
}