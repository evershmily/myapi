<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>inputid</name>
   <tag></tag>
   <elementGuidId>11236ce9-3852-4f1b-b96f-017d45f4f7ea</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;token\&quot;: \&quot;${id}\&quot;}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://hr.aetest.bwae.org/kpi-salary-pc/RPC406CFAFEC8BD5BA8973308A3E1358AC9</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>findTestData('id').getValue(1, 1)</defaultValue>
      <description></description>
      <id>9eca169e-149c-4a76-b063-e06075f619d2</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyElementPropertyValue(response, 'd', &quot;SUCCESS&quot;)
def JsonSlurper= new JsonSlurper()
def jsonresponse=JsonSlurper.parseText(response.getResponseText())
def cookie=response.getHeaderFields()
def a=cookie.get('Set-Cookie')[0].split(&quot;;&quot;)[0]
println(a)
File file=new File(&quot;D://auto/APItest/Data Files/cookie.csv&quot;)
BufferedWriter out=new BufferedWriter(new FileWriter(file))
out.write(a)
out.flush()
out.close()



</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
