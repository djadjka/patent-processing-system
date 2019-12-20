#  Patent processing system
 Данная система создается как api для веб интерфейса, в котором пользователь на основе данных полученных с сайта [USPTO](https://www.uspto.gov/) подать заявку на продление патента. 
## Подход 

В данной системе будет использоваться подход неклассический Rest т.к он лучше всего подходит для веб системы с веб интерфейсом, и соответствует таким требованиям.

 - Модель клиент-сервер
 - Отсутствие состояния
 - Кэширование
 - Каждый документ имеет идентификатор 
 - Может являться  многоуровневой системой 
 - Может возвращать статику в виде веб интерфейса для работы с этими данными 
 
 При этом не будут использоватся такие свойства как:
 - self-descriptive
 - hypermedia
 
т.к в моем случае это будет лишней нагрузкой, т.к веб интерфейс тоже будет разработан мной. 
Далее, при большой необходимости дополнительных интеграций, можно будет рассмотреть возможность реализации последних 2 пунктов 

## Запросы
Описание: получить инофрмацию о патенте 
URL: /Patents/{serialNumber}  
Метод: GET  
Параметры: serialNumber: String - номер патента в uspto  
Ответ: 
```yaml
{
   serialNumber: String, // номер патента в uspto
   registrationDate: Number, // вермя в секундак начиная с 1 января 1970
   expireDate: Number, // вермя в секундак начиная с 1 января 1970
   company: String // Название фирмы,
   img: String // url лого,
   info: String // описание патента 
}
```
___
Описание: положить информацию о патенте в систему  
URL: /Patents  
Метод: POST  
Параметры: 
```yaml
{
   serialNumber: String, // номер патента в uspto
   registrationDate: Number, // вермя в секундак начиная с 1 января 1970
   expireDate: Number, // вермя в секундак начиная с 1 января 1970
   company: String // Название фирмы,
   img: String // url лого,
   info: String // описание патента 
}
```
Ответ: путой ответ
___
Описание: добавить заявку на продление   
URL: /Requests  
Метод: POST  
Параметры:   
```yaml
{
    serialNumber: String, // номер патента в uspto
    infoUpdate?: String, // обновленное описание патента 
    img?: String: // url нового лого 
}
```
Ответ: id: Number - id заявки в системе 
___
Описание: получить инофрмацию о заявки    
URL: /Requests/{id}  
Метод: GET  
Параметры: id: Number - id заявки в системе   
Ответ:
```yaml
{
    serialNumber: String, // номер патента в uspto
    infoUpdate?: String,  // обновленное описание патента 
    status: String // [sent, on_the_finalization, waiting, completed ],
    img?: String  // url нового лого 
}
```
___
Описание: обновить информацию заявки    
URL: /Requests/{id}   
Метод: PUT   
Параметры: id: Number - id заявки в системе    
```yaml
{
   infoUpdate?: String, // обновленное описание патента 
   img?: String: // url нового лого 
}
```
Ответ: путой ответ
___
Описание: обновить статус заявки   
URL: /Requests/{id}/status    
Метод: PUT   
Параметры: id: Number - id заявки в системе   
```yaml
{
   status: String // [sent, on_the_finalization, waiting, completed ]
}
```
Ответ: путой ответ
