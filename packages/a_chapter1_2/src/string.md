##문자열
String과 str은 비슷해보이지만 별개의 유형이다.<br/>
###String
*String*은 합치거나 추가, 공백 제거와 같은 다양한 기능들이 있다.<br/>
String은 awned(소유) type이다. 소유자는 데이터를 변경할 수 있으며 범위를 벗어날 때 소유한 값을 삭제할 책임이 있습니다.<br/>
String::from(&str)로 &str -> String 변환 가능
---
###str
*str*은 기능이 적지만 고성능 유형이다. 한번 생성된 str은 확장할 수 없다. 마치 원시 메모리(c언어에서의 배열과 같은) 유사하지만 Rust는 UTF-8 문자를 보장한다. 
str은 일반적으로 &str처럼 표시된다.
string slice라고 부른다.
<br/>
str유형에 변수를 할당하면 실패한다. str 값은 임의의 길이일 수 있으므로 참조에 의해 지역 변수로만 저장할 수 있습니다.
<br/>
&str은 borrowed(빌린) type이다. 실용적인 측면에서 이것은 &str이 읽기 전용 데이터로 간주될 수 있는 반면 String은 읽기-쓰기로 간주될 수 있음을 의미합니다.
---
###String literals
"hello world" 같은 String literals는 &`static str
처럼
---
###char
4바이트로 된 단일 문자.
1~4바이트로 이루어져있는 UTF-8에 비해 컴파일러가 추론하기 쉽다.
---
String은 동적 메모리 할당을 사용하여 표시하는 텍스트를 저장합니다. &str 값을 생성하면 메모리 할당을 피할 수 있다.
<br/>

Vec&lt;u8&gt; - 일반적으로 [u8] 데이터를 사용할 때 생성되는 원시 바이트 벡터입니다. String은 Vec&gt;u8&lt;에 대한 것이고 str은 [u8]에 대한 것입니다.