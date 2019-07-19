<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>BOModCompSarCal</name>
   <tag></tag>
   <elementGuidId>5606b731-6649-4b16-ab8b-6f06d78ad126</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;objid\&quot;: \&quot;PHMODSARCO\&quot;,\n  \&quot;clntnum\&quot;: \&quot;52531592\&quot;,\n  \&quot;currcy\&quot;: \&quot;IDR\&quot;,\n  \&quot;crtable01\&quot;: \&quot;U1BR\&quot;,\n  \&quot;rcdate\&quot;: \&quot;20080727\&quot;,\n  \&quot;rcesagenew\&quot;: \&quot;0\&quot;,\n  \&quot;rcestrmnew\&quot;: \&quot;99\&quot;,\n  \&quot;suminsnew\&quot;: \&quot;00000004000000000\&quot;,\n  \&quot;zsprm\&quot;: \&quot;00000000027083300\&quot;,\n  \&quot;chdrnum\&quot;: \&quot;31060667\&quot;,\n  \&quot;rider\&quot;: \&quot;00\&quot;,\n  \&quot;crtable02\&quot;: \&quot;U1BR\&quot;\n}\n&quot;,
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
   <restUrl>{endpoint}/phs-sarcalculator/rest/api/BOModCompSarCal</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>6b2fecac-0536-4770-8cac-f6282dd90219</id>
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
