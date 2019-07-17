<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>BOAddPolSarCal</name>
   <tag></tag>
   <elementGuidId>b8d72348-b874-48d7-8bf8-a4202c08c4ac</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;objid\&quot;: \&quot;PHADDSARPO\&quot;,\n  \&quot;clntnum\&quot;: \&quot;60074153\&quot;,\n  \&quot;currcy\&quot;: \&quot;IDR\&quot;,\n  \&quot;chdrnum\&quot;: \&quot;90112763\&quot;,\n  \&quot;crtable\&quot;: \&quot;U31R\&quot;,\n  \&quot;rcdate\&quot;: \&quot;99999999\&quot;,\n  \&quot;rcesagenew\&quot;: \&quot;55\&quot;,\n  \&quot;rcestrmnew\&quot;: \&quot;0\&quot;,\n  \&quot;suminsnew\&quot;: \&quot;00000001000000000\&quot;\n}\n&quot;,
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
   <restUrl>{endpoint}/phs-bosarcalculator/rest/api/BOAddPolSarCal</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>25299225-6894-4f57-9911-35acf6def6b1</id>
      <masked>false</masked>
      <name>endpoint</name>
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
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
