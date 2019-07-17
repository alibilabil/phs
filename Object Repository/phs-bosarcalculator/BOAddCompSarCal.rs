<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>BOAddCompSarCal</name>
   <tag></tag>
   <elementGuidId>c188f084-43b9-40cb-a5e3-ba0a7fd0fa9d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;objid\&quot;: \&quot;PHADDSARCO\&quot;,\n  \&quot;clntnum\&quot;: \&quot;60072770\&quot;,\n  \&quot;currcy\&quot;: \&quot;IDR\&quot;,\n  \&quot;crtable01\&quot;: \&quot;S1GR\&quot;,\n  \&quot;rcesagenew\&quot;: \&quot;0\&quot;,\n  \&quot;rcestrmnew\&quot;: \&quot;25\&quot;,\n  \&quot;suminsnew\&quot;: \&quot;00000000020000000\&quot;,\n  \&quot;zsprm\&quot;: \&quot;00000000200000000\&quot;,\n  \&quot;chdrnum\&quot;: \&quot;90002667\&quot;,\n  \&quot;rider\&quot;: \&quot;\&quot;,\n  \&quot;crtable02\&quot;: \&quot;S1GR\&quot;\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-sarcalculator/rest/api/BOAddCompSarCal</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>de3fa071-bb03-47a1-92c4-6cd2569f17e8</id>
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
