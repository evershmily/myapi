import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import org.openqa.selenium.WebDriver as WebDriver
import org.openqa.selenium.Cookie as Cookie
import com.kms.katalon.core.testobject.RequestObject as RequestObject
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject
import com.kms.katalon.core.webservice.verification.WSResponseManager as WSResponseManager
import groovy.json.JsonSlurper as JsonSlurper

not_run: response = WS.sendRequest(findTestObject('login'))

not_run: WS.verifyResponseStatusCode(response, 200)

response = WS.sendRequest(findTestObject('inputid', [('id') : id]))

def cookie = response.getHeaderFields()

def a = cookie.get('Set-Cookie')[0].split(';')[0]

println(a)

/*File file = new File('D://auto/APItest/Data Files/cookie.csv')

BufferedWriter out = new BufferedWriter(new FileWriter(file))

out.write(a)

out.flush()

out.close()*/
WS.delay(2)

WS.verifyResponseStatusCode(response, 200)

//response = WS.sendRequest(findTestObject('New Request', [('cookie') : findTestData('cookie (1)').getValue(1, 1)]))
response = WS.sendRequest(findTestObject('New Request', [('cookie') : a]))

WS.verifyResponseStatusCode(response, 200)

WS.verifyElementPropertyValue(response, 'd.assessmentTime.id', '68')

response = WS.sendRequest(findTestObject('kkk', [('cookie') : findTestData('cookie (1)').getValue(1, 1), ('name') : name]))

WS.verifyElementPropertyValue(response, 'd.userName', name)

println(response.getResponseBodyContent())

