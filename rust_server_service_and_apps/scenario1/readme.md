웹 서버에는 _서버, 라우터, 핸들러 및 HTTP 라이브러리의_ 네 가지 구성 요소가 있습니다.

이러한 각 구성 요소에는 SRP(단일 책임 원칙)에 따라 특정 목적이 있습니다.

서버는 들어오는 TCP Byte stream을 수신합니다. 
HTTP 라이브러리는 Byte stream을 해석하고 이를 HTTP Request(Message)로 변환합니다.

라우터는 HTTP Request를 수락하고 호출할 핸들러를 결정합니다.

핸들러는 HTTP Request를 처리하고 HTTP Response를 구성합니다.

HTTP Response(Message)는 HTTP 라이브러리를 사용하여 다시 Byte stream으로 변환된 다음 클라이언트로 다시 전송됩니다.

## Server

## Router

## Handler 

## HTTP Library