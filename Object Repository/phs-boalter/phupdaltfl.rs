<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>phupdaltfl</name>
   <tag></tag>
   <elementGuidId>faef31c7-1a42-476f-b18c-930baa967dee</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;objid\&quot;: \&quot;${objid}\&quot;,\n  \&quot;chdrsel\&quot;: \&quot;${chdrsel}\&quot;,\n  \&quot;zaltnumsel\&quot;: \&quot;${zaltnumsel}\&quot;,\n  \&quot;zaltstasel\&quot;: \&quot;${zaltstasel}\&quot;,\n  \&quot;zalttypsel\&quot;: \&quot;${zalttypsel}\&quot;,\n  \&quot;zaltdesc\&quot;: \&quot;${zaltdesc}\&quot;,\n  \&quot;zlastrcvdt\&quot;: \&quot;${zlastrcvdt}\&quot;,\n  \&quot;zcompldt\&quot;: \&quot;${zcompldt}\&quot;,\n  \&quot;fupcde\&quot;: \&quot;${fupcde}\&quot;,\n  \&quot;clntnum\&quot;: \&quot;${clntnum}\&quot;,\n  \&quot;fupdt\&quot;: \&quot;${fupdt}\&quot;,\n  \&quot;fupno\&quot;: \&quot;${fupno}\&quot;,\n  \&quot;fuprmk\&quot;: \&quot;${fuprmk}\&quot;,\n  \&quot;fupsts\&quot;: \&quot;${fupsts}\&quot;,\n  \&quot;delflag\&quot;: \&quot;${fupsts}\&quot;,\n  \&quot;addopt\&quot;: \&quot;${addopt}\&quot;,\n  \&quot;ynflag\&quot;: \&quot;${ynflag}\&quot;\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-boalteration/rest/api/phupdaltfl</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>3477746e-07f0-4561-bab2-07d3f3c57361</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-boalteration/phupdaltfl').getValue(1, 1)</defaultValue>
      <description></description>
      <id>79fafaa6-5e32-4440-aa70-6c0d7e8c5c1f</id>
      <masked>false</masked>
      <name>objid</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-boalteration/phupdaltfl').getValue(2, 1)</defaultValue>
      <description></description>
      <id>dc097241-6d09-4701-b8c3-f9a3b733c23b</id>
      <masked>false</masked>
      <name>chdrsel</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-boalteration/phupdaltfl').getValue(3, 1)</defaultValue>
      <description></description>
      <id>3bdd5ccb-9965-412f-b894-85c707b63259</id>
      <masked>false</masked>
      <name>zaltnumsel</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-boalteration/phupdaltfl').getValue(4, 1)</defaultValue>
      <description></description>
      <id>5672f61c-1ef8-4858-9f20-7ad961037904</id>
      <masked>false</masked>
      <name>zaltstasel</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-boalteration/phupdaltfl').getValue(5, 1)</defaultValue>
      <description></description>
      <id>44edcfeb-0025-44e4-85f9-2464aaf6e36c</id>
      <masked>false</masked>
      <name>zalttypsel</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-boalteration/phupdaltfl').getValue(6, 1)</defaultValue>
      <description></description>
      <id>108c393c-01c0-4765-8684-cbc92c3236b5</id>
      <masked>false</masked>
      <name>zaltdesc</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-boalteration/phupdaltfl').getValue(7, 1)</defaultValue>
      <description></description>
      <id>ec3b1bc2-b542-4548-9dfd-3dc084c66d5f</id>
      <masked>false</masked>
      <name>zlastrcvdt</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-boalteration/phupdaltfl').getValue(8, 1)</defaultValue>
      <description></description>
      <id>3bea88ec-9f18-4559-b23a-59820df60a72</id>
      <masked>false</masked>
      <name>zcompldt</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-boalteration/phupdaltfl').getValue(9, 1)</defaultValue>
      <description></description>
      <id>1e09fb1d-a215-4777-8e04-63220eaf8252</id>
      <masked>false</masked>
      <name>fupcde</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-boalteration/phupdaltfl').getValue(10, 1)</defaultValue>
      <description></description>
      <id>50934e98-c635-4a9b-93e8-478065163b58</id>
      <masked>false</masked>
      <name>clntnum</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-boalteration/phupdaltfl').getValue(11, 1)</defaultValue>
      <description></description>
      <id>d0103185-8408-4a2e-8f61-f66c3fd1a746</id>
      <masked>false</masked>
      <name>fupdt</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-boalteration/phupdaltfl').getValue(12, 1)</defaultValue>
      <description></description>
      <id>23752e3c-707a-4bb4-aa82-d786894e9b24</id>
      <masked>false</masked>
      <name>fupno</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-boalteration/phupdaltfl').getValue(13, 1)</defaultValue>
      <description></description>
      <id>5ed6e7a8-06e6-4739-a116-d2f3cb85a93f</id>
      <masked>false</masked>
      <name>fuprmk</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-boalteration/phupdaltfl').getValue(14, 1)</defaultValue>
      <description></description>
      <id>1f2f5a06-6a90-4249-b6fe-3cb9d2a837a6</id>
      <masked>false</masked>
      <name>fupsts</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-boalteration/phupdaltfl').getValue(15, 1)</defaultValue>
      <description></description>
      <id>21fdb4c7-4f3f-4c9a-a472-f829f706165c</id>
      <masked>false</masked>
      <name>delflag</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-boalteration/phupdaltfl').getValue(16, 1)</defaultValue>
      <description></description>
      <id>d51ae51b-ec7c-4149-a49a-52954decc06f</id>
      <masked>false</masked>
      <name>addopt</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-boalteration/phupdaltfl').getValue(17, 1)</defaultValue>
      <description></description>
      <id>3276d60c-7649-492a-b726-4a3314d0c5b5</id>
      <masked>false</masked>
      <name>ynflag</name>
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
