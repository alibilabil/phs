<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>BORetriveAddExclusion</name>
   <tag></tag>
   <elementGuidId>7e2a8b7f-997c-4f78-b2fe-c3d2f0ec1953</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;CHDRNUM\&quot;: \&quot;90000252\&quot;,\n  \&quot;ZALTNUM\&quot;: \&quot;A0000007\&quot;,\n  \&quot;FUPNO\&quot;: \&quot;00012\&quot;,\n  \&quot;FUPCDE\&quot;: \&quot;MNR\&quot;,\n  \&quot;HXCLLETTYP\&quot;: \&quot;\&quot;,\n  \&quot;HXCLNOTE1\&quot;: \&quot;abcdefghijklmnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnn233333333333nz123123123121231aaaaa\&quot;,\n  \&quot;HXCLNOTE2\&quot;: \&quot;Tes Tes satu dua tiga 123\&quot;,\n  \&quot;OBJID\&quot;: \&quot;PHADDEXCLS\&quot;\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-retrieve/rest/api/BORetriveAddExclusion</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>3a486d0e-7f2b-4ae2-b728-395db743fc73</id>
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
