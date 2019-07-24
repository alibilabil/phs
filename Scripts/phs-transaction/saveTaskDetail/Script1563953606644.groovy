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
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

ResponseObject response = WS.sendRequest(findTestObject('phs-transaction/saveTaskDetail', [('endpoint') : GlobalVariable.endpoint, ('trxId') : findTestData(
                'phs-transaction/saveTaskDetail').getValue(1, 1), ('genId') : findTestData('phs-transaction/saveTaskDetail').getValue(
                2, 1), ('count') : findTestData('phs-transaction/saveTaskDetail').getValue(3, 1), ('statusDelete') : findTestData(
                'phs-transaction/saveTaskDetail').getValue(4, 1), ('verify') : findTestData('phs-transaction/saveTaskDetail').getValue(
                5, 1), ('tskdetailId') : findTestData('phs-transaction/saveTaskDetail').getValue(6, 1), ('processId') : findTestData(
                'phs-transaction/saveTaskDetail').getValue(7, 1), ('processInstanceName') : findTestData('phs-transaction/saveTaskDetail').getValue(
                8, 1), ('displayName') : findTestData('phs-transaction/saveTaskDetail').getValue(9, 1), ('taskId') : findTestData(
                'phs-transaction/saveTaskDetail').getValue(10, 1), ('startTime') : findTestData('phs-transaction/saveTaskDetail').getValue(
                11, 1), ('trxIn') : findTestData('phs-transaction/saveTaskDetail').getValue(12, 1), ('trxUpdate') : findTestData(
                'phs-transaction/saveTaskDetail').getValue(13, 1), ('processFlow') : findTestData('phs-transaction/saveTaskDetail').getValue(
                14, 1), ('status') : findTestData('phs-transaction/saveTaskDetail').getValue(15, 1), ('role') : findTestData(
                'phs-transaction/saveTaskDetail').getValue(16, 1), ('roleDistribution') : findTestData('phs-transaction/saveTaskDetail').getValue(
                17, 1), ('taskName') : findTestData('phs-transaction/saveTaskDetail').getValue(18, 1), ('decision') : findTestData(
                'phs-transaction/saveTaskDetail').getValue(19, 1), ('username') : findTestData('phs-transaction/saveTaskDetail').getValue(
                20, 1), ('docId') : findTestData('phs-transaction/saveTaskDetail').getValue(21, 1)]))

WS.verifyResponseStatusCode(response, 200)