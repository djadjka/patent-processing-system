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
 
## Ресурсы 
### Patent:
Данные о патенте полученные с сайта uspto
```json
{
	serialNumber: String,
	registrationDate: Timestamp,
	expireDate: Date,
	company: String
}
```

### Request:
Заявка пользователя на продление 
```json
{
	id:String 
	serialNumber: String,
	contactPhone: String,
	status: String
}
```

## Запросы
	 /Patents/{serialNumber}
Get - получить информацию о патенте 

	 /Patents
Post - создать информацию о патенте 

	 /Requests/{id}
Get - получить запрос на продление

Put - обновить данные о патенте 

	 /Requests
Post - создать заявку на продление 


